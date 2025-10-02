use std::sync::atomic::{AtomicUsize, Ordering};
use std::cell::UnsafeCell;
use std::mem::MaybeUninit;

// ============================================================================
// Multi-Producer Single-Consumer Queue (MPSC)
// Multiple threads push, single thread pops - NO contention on reads!
// ============================================================================
pub struct PublishQueue<T> {
    buffer: Box<[UnsafeCell<MaybeUninit<T>>]>,
    write_pos: AtomicUsize,
    read_pos: AtomicUsize, // Only accessed by single consumer
    capacity: usize,
}

unsafe impl<T: Send> Send for PublishQueue<T> {}
unsafe impl<T: Send> Sync for PublishQueue<T> {}

impl<T> PublishQueue<T> {
    pub fn new(capacity: usize) -> Self {
        assert!(capacity > 0 && capacity.is_power_of_two(), 
                "Capacity must be power of 2 for fast modulo");
        
        let mut buffer = Vec::with_capacity(capacity);
        buffer.resize_with(capacity, || UnsafeCell::new(MaybeUninit::uninit()));
        
        Self {
            buffer: buffer.into_boxed_slice(),
            write_pos: AtomicUsize::new(0),
            read_pos: AtomicUsize::new(0),
            capacity,
        }
    }
    
    /// Push item from any producer thread
    /// Returns Err(item) if queue is full
    pub fn push(&self, item: T) -> Result<(), T> {
        loop {
            let write = self.write_pos.load(Ordering::Acquire);
            let read = self.read_pos.load(Ordering::Acquire);
            
            let next_write = (write + 1) & (self.capacity - 1); // Fast modulo
            
            // Check if full
            if next_write == read {
                return Err(item);
            }
            
            // Try to claim slot atomically
            if self.write_pos.compare_exchange_weak(
                write,
                next_write,
                Ordering::AcqRel,
                Ordering::Acquire
            ).is_ok() {
                unsafe {
                    (*self.buffer[write].get()).write(item);
                }
                return Ok(());
            }
            // CAS failed, retry
        }
    }
    
    /// Pop item - ONLY call from single consumer thread!
    pub fn pop(&self) -> Option<T> {
        // No contention here - only one thread reads
        let read = self.read_pos.load(Ordering::Relaxed); // Can use Relaxed!
        let write = self.write_pos.load(Ordering::Acquire);
        
        if read == write {
            return None; // Empty
        }
        
        let item = unsafe {
            (*self.buffer[read].get()).assume_init_read()
        };
        
        let next_read = (read + 1) & (self.capacity - 1);
        self.read_pos.store(next_read, Ordering::Release);
        
        Some(item)
    }
    
    /// Drain multiple items at once - efficient for event loops
    pub fn drain_batch(&self, max: usize) -> Vec<T> {
        let mut batch = Vec::with_capacity(max);
        
        for _ in 0..max {
            match self.pop() {
                Some(item) => batch.push(item),
                None => break,
            }
        }
        
        batch
    }
    
    pub fn len(&self) -> usize {
        let write = self.write_pos.load(Ordering::Acquire);
        let read = self.read_pos.load(Ordering::Acquire);
        
        if write >= read {
            write - read
        } else {
            self.capacity - read + write
        }
    }
    
    pub fn is_empty(&self) -> bool {
        self.read_pos.load(Ordering::Acquire) == 
            self.write_pos.load(Ordering::Acquire)
    }
}

// ============================================================================
// Single-Producer Multi-Consumer Queue (SPMC)
// Single thread pushes, multiple threads pop - NO contention on writes!
// ============================================================================
pub struct FinishQueue<T> {
    buffer: Box<[UnsafeCell<MaybeUninit<T>>]>,
    write_pos: AtomicUsize, // Only accessed by single producer
    read_pos: AtomicUsize,
    capacity: usize,
}

unsafe impl<T: Send> Send for FinishQueue<T> {}
unsafe impl<T: Send> Sync for FinishQueue<T> {}

impl<T> FinishQueue<T> {
    pub fn new(capacity: usize) -> Self {
        assert!(capacity > 0 && capacity.is_power_of_two(), 
                "Capacity must be power of 2 for fast modulo");
        
        let mut buffer = Vec::with_capacity(capacity);
        buffer.resize_with(capacity, || UnsafeCell::new(MaybeUninit::uninit()));
        
        Self {
            buffer: buffer.into_boxed_slice(),
            write_pos: AtomicUsize::new(0),
            read_pos: AtomicUsize::new(0),
            capacity,
        }
    }
    
    /// Push item - ONLY call from single producer thread!
    pub fn push(&self, item: T) -> Result<(), T> {
        // No contention here - only one thread writes
        let write = self.write_pos.load(Ordering::Relaxed); // Can use Relaxed!
        let read = self.read_pos.load(Ordering::Acquire);
        
        let next_write = (write + 1) & (self.capacity - 1);
        
        // Check if full
        if next_write == read {
            return Err(item);
        }
        
        unsafe {
            (*self.buffer[write].get()).write(item);
        }
        
        self.write_pos.store(next_write, Ordering::Release);
        Ok(())
    }
    
    /// Pop item from any consumer thread
    pub fn pop(&self) -> Option<T> {
        loop {
            let read = self.read_pos.load(Ordering::Acquire);
            let write = self.write_pos.load(Ordering::Acquire);
            
            if read == write {
                return None; // Empty
            }
            
            // Try to claim slot atomically
            let next_read = (read + 1) & (self.capacity - 1);
            
            if self.read_pos.compare_exchange_weak(
                read,
                next_read,
                Ordering::AcqRel,
                Ordering::Acquire
            ).is_ok() {
                let item = unsafe {
                    (*self.buffer[read].get()).assume_init_read()
                };
                return Some(item);
            }
            // CAS failed, retry
        }
    }
    
    /// Batch push - efficient for single producer
    pub fn push_batch(&self, items: impl IntoIterator<Item = T>) -> Result<(), Vec<T>> {
        let mut failed = Vec::new();
        
        for item in items {
            if let Err(item) = self.push(item) {
                failed.push(item);
            }
        }
        
        if failed.is_empty() {
            Ok(())
        } else {
            Err(failed)
        }
    }
    
    pub fn len(&self) -> usize {
        let write = self.write_pos.load(Ordering::Acquire);
        let read = self.read_pos.load(Ordering::Acquire);
        
        if write >= read {
            write - read
        } else {
            self.capacity - read + write
        }
    }
    
    pub fn is_empty(&self) -> bool {
        self.read_pos.load(Ordering::Acquire) == 
            self.write_pos.load(Ordering::Acquire)
    }
}

impl<T> Drop for PublishQueue<T> {
    fn drop(&mut self) {
        while self.pop().is_some() {}
    }
}

impl<T> Drop for FinishQueue<T> {
    fn drop(&mut self) {
        while self.pop().is_some() {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::thread;
    
    #[test]
    fn test_mpsc_publish_queue() {
        let queue = Arc::new(PublishQueue::new(1024));
        let mut handles = vec![];
        
        // Spawn 4 producer threads
        for i in 0..4 {
            let q = queue.clone();
            handles.push(thread::spawn(move || {
                for j in 0..100 {
                    while q.push(i * 1000 + j).is_err() {
                        thread::yield_now();
                    }
                }
            }));
        }
        
        // Single consumer
        let q = queue.clone();
        let consumer = thread::spawn(move || {
            let mut count = 0;
            while count < 400 {
                if let Some(_item) = q.pop() {
                    count += 1;
                }
            }
            count
        });
        
        for h in handles {
            h.join().unwrap();
        }
        
        assert_eq!(consumer.join().unwrap(), 400);
    }
    
    #[test]
    fn test_spmc_finish_queue() {
        let queue = Arc::new(FinishQueue::new(1024));
        
        // Single producer
        let q = queue.clone();
        let producer = thread::spawn(move || {
            for i in 0..400 {
                while q.push(i).is_err() {
                    thread::yield_now();
                }
            }
        });
        
        // Spawn 4 consumer threads
        let mut handles = vec![];
        for _ in 0..4 {
            let q = queue.clone();
            handles.push(thread::spawn(move || {
                let mut count = 0;
                loop {
                    if let Some(_item) = q.pop() {
                        count += 1;
                    } else if count > 50 { // Got some items
                        break;
                    }
                }
                count
            }));
        }
        
        producer.join().unwrap();
        
        let total: usize = handles.into_iter()
            .map(|h| h.join().unwrap())
            .sum();
        
        assert_eq!(total, 400);
    }
}