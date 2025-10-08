use crate::error::MbError;
use crate::error::MbMgrErrorKind;
use crate::job::JobStatus;
use crate::mgr::MbMgr;
use std::collections::VecDeque;
use std::marker::PhantomData;
use std::rc::Rc;
use std::sync::mpsc;
use crate::job::MbJobHandle;
use intel_ipsec_mb_sys::ImbStatus;
use std::thread;
use crate::operation::Operation;

#[derive(Debug)]
pub struct MbRuntime {
    mgr: MbMgr,
    job_rx: mpsc::Receiver<MbJobRequest>, // Receive from other threads
    job_queue: VecDeque<MbJobRequest>,
    _not_thread_safe: PhantomData<Rc<()>>,
}

#[derive(Debug)]
pub struct MbRuntimeHandle {
    pub join_handle: thread::JoinHandle<()>,
    job_tx: mpsc::Sender<MbJobRequest>,
}

/// Uninitialized runtime - Send + Sync (can be moved to worker thread)
#[derive(Debug)]
pub struct MbRuntimeInit {
    job_rx: mpsc::Receiver<MbJobRequest>,
    capacity: usize,
}

pub(crate) struct MbJobRequest {
    pub handle: Option<MbJobHandle<'static>>,
    pub operation: Box<dyn Operation<'static> + Send>,
    pub completion: mpsc::SyncSender<JobStatus>,
}

impl std::fmt::Debug for MbJobRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MbJobRequest")
            .field("operation", &"<Operation>")
            .field("completion", &self.completion)
            .finish()
    }
}

impl MbRuntime {
    fn run_loop(&mut self) {
        loop {
            let job_request = self.job_rx.recv();
            match job_request {
                Ok(mb_job_request) => {
                    // while let Ok(job) = self.job_rx.try_recv() {
                    //     self.job_queue.push_back(job);
                    //     if self.job_queue.len() >= 64 {
                    //         break;
                    //     }
                    // }

                    // self.process_batch();
                    self.process_job(mb_job_request);

                   
                }
                Err(_) => break,
            }
        }
    }

    // fn process_batch(&mut self) {
    //     while let Some(job) = self.job_queue.pop_front() {
    //         let status = self.process_job(job.operation);
    //         let _ = job.completion.send(status);
    //     }
    // }

    fn process_job(&mut self, mut job_request: MbJobRequest) {
        let (handle, did_last_job_finish) = self.mgr.handoff_job(&mut *job_request.operation).unwrap();
        job_request.handle = Some(handle);
        self.job_queue.push_back(job_request);

        if did_last_job_finish {
            let prev_job_request = self.job_queue.pop_front().unwrap();
            prev_job_request.handle.unwrap().resolve().unwrap();
            prev_job_request.completion.send(JobStatus{status: ImbStatus::IMB_STATUS_COMPLETED}).unwrap();
        }

        if self.should_flush() {
            self.flush();
        }
  
    }

    fn should_flush(&self) -> bool {
        false 
    }

    fn flush(&mut self) {
        unsafe {
            self.mgr.flush_job().unwrap();
        }
    }
}


impl MbRuntimeHandle {
    /// Submit a job with scoped lifetime guarantees
    ///
    /// This method blocks until the job completes, ensuring that any
    /// borrowed data in the operation remains valid throughout execution.
    pub fn submit_job<'scope>(
        &self,
        operation: impl Operation<'scope> + Send + 'scope,
    ) -> Result<JobStatus, MbError> {
        let (completion_tx, completion_rx) = mpsc::sync_channel(1);

        // Box the operation with its actual lifetime
        let boxed: Box<dyn Operation<'scope> + Send + 'scope> = Box::new(operation);

        // Transmute to 'static for storage
        // SAFETY: This is safe because:
        // 1. We block on completion_rx.recv() below
        // 2. The operation completes before this function returns
        // 3. Therefore, borrowed data ('scope) remains valid throughout
        let erased: Box<dyn Operation<'static> + Send> = unsafe { std::mem::transmute(boxed) };

        let request = MbJobRequest {
            handle: None,
            operation: erased,
            completion: completion_tx,
        };

        self.job_tx
            .send(request)
            .map_err(|_| MbError::from_kind(MbMgrErrorKind::ChannelClosed))?;

        // CRITICAL: Blocking here enforces the lifetime guarantee
        completion_rx
            .recv()
            .map_err(|_| MbError::from_kind(MbMgrErrorKind::CompletionFailed))
    }
}

impl MbRuntimeInit {
    /// Initialize MbMgr on the current thread and start processing
    /// This consumes self and creates the !Send runtime
    pub fn run(self) {
        // Initialize MbMgr HERE, on the worker thread
        let mgr = MbMgr::new().expect("Failed to initialize MbMgr");

        // Now create the actual runtime (which is !Send)
        let mut runtime = MbRuntime {
            mgr,
            job_rx: self.job_rx,
            job_queue: VecDeque::with_capacity(self.capacity),
            _not_thread_safe: PhantomData,
        };

        // Run the processing loop
        runtime.run_loop();
    }
}

/// Create and spawn runtime
pub fn spawn_runtime() -> MbRuntimeHandle {
    spawn_runtime_with_capacity(128)
}

pub fn spawn_runtime_with_capacity(capacity: usize) -> MbRuntimeHandle {
    let (job_tx, job_rx) = mpsc::channel();

    let init = MbRuntimeInit { job_rx, capacity };

    // Move the uninitialized runtime to worker thread
    let join_handle = std::thread::spawn(move || {
        init.run(); // ✓ Works! MbRuntimeInit is Send
    });

    MbRuntimeHandle { job_tx, join_handle: join_handle }
}
