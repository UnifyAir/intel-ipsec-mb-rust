// use std::mem::MaybeUninit;
// use std::marker::PhantomData;
// use intel_ipsec_mb_sys::IMB_MAX_JOBS;
// use crate::job::MbJobHandle;



// /// Ring buffer for tracking in-flight IMB cryptographic jobs.
// /// Leverages Intel IPSec MB library's guarantee of in-order completion.
// pub struct JobRing<const CAPACITY: usize = IMB_MAX_JOBS>
// {
//     ring: [MaybeUninit<MbJobHandle>; CAPACITY],
//     head: usize,  // Where next job will be written (submit position)
//     tail: usize,  // Where next completed job will be read (completion position)
//     // _phantom: PhantomData<F>,
// }

// impl<const CAPACITY: usize> JobRing<CAPACITY>
// {
//     /// Creates a new empty job ring.
//     #[inline]
//     pub const fn new() -> Self {
//         Self {
//             ring: unsafe { MaybeUninit::uninit().assume_init() },
//             head: 0,
//             tail: 0,
//             // _phantom: PhantomData,
//         }
//     }

//     /// Returns the number of jobs currently pending.
//     #[inline]
//     pub const fn pending(&self) -> usize {
//         if self.head >= self.tail {
//             self.head - self.tail
//         } else {
//             CAPACITY - self.tail + self.head
//         }
//     }

//     /// Returns true if no jobs are pending.
//     #[inline]
//     pub const fn is_empty(&self) -> bool {
//         self.head == self.tail
//     }

//     /// Returns true if the ring is at capacity.
//     #[inline]
//     pub const fn is_full(&self) -> bool {
//         self.pending() >= CAPACITY
//     }

//     /// Returns the maximum number of jobs that can be queued.
//     #[inline]
//     pub const fn capacity(&self) -> usize {
//         CAPACITY
//     }

//     /// Enqueues a job handle into the ring.
//     /// 
//     /// # Errors
//     /// Returns the handle back if ring is full.
//     #[inline]
//     pub fn enqueue(&mut self, handle: MbJobHandle) -> Result<(), MbJobHandle> {
//         if self.is_full() {
//             return Err(handle);
//         }

//         unsafe {
//             self.ring[self.head].write(handle);
//         }
//         self.head = (self.head + 1) % CAPACITY;
//         Ok(())
//     }

//     /// Enqueues a job handle without checking capacity.
//     /// 
//     /// # Safety
//     /// Caller must ensure ring is not full via `is_full()` check.
//     #[inline]
//     pub unsafe fn enqueue_unchecked(&mut self, handle: MbJobHandle) {
//         self.ring[self.head].write(handle);
//         self.head = (self.head + 1) % CAPACITY;
//     }

//     /// Dequeues a completed job handle from the ring.
//     /// 
//     /// Returns `None` if ring is empty.
//     #[inline]
//     pub fn dequeue(&mut self) -> Option<MbJobHandle> {
//         if self.is_empty() {
//             return None;
//         }

//         unsafe {
//             let handle = self.ring[self.tail].assume_init_read();
//             self.tail = (self.tail + 1) % CAPACITY;
//             Some(handle)
//         }
//     }

//     /// Dequeues a job handle without checking if empty.
//     /// 
//     /// # Safety
//     /// Caller must ensure ring is not empty via `is_empty()` check.
//     #[inline]
//     pub unsafe fn dequeue_unchecked(&mut self) -> MbJobHandle {
//         let handle = self.ring[self.tail].assume_init_read();
//         self.tail = (self.tail + 1) % CAPACITY;
//         handle
//     }

//     /// Clears all pending jobs from the ring.
//     pub fn clear(&mut self) {
//         while !self.is_empty() {
//             unsafe {
//                 self.ring[self.tail].assume_init_drop();
//                 self.tail = (self.tail + 1) % CAPACITY;
//             }
//         }
//     }
// }

// impl<const CAPACITY: usize> Drop for JobRing<CAPACITY>
// {
//     fn drop(&mut self) {
//         self.clear();
//     }
// }

// // unsafe impl<F: Send, const CAPACITY: usize> Send for JobRing<F, CAPACITY> where F: FnOnce(&[u8]) {}