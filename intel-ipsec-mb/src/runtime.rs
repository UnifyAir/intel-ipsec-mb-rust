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

#[cfg(feature = "async")]
use tokio::sync::oneshot;

#[derive(Debug)]
pub(crate) enum JobCompletion {
    Sync(mpsc::SyncSender<JobStatus>),
    #[cfg(feature = "async")]
    Async(oneshot::Sender<JobStatus>),
}

impl JobCompletion {
    fn send(self, status: JobStatus) -> Result<(), MbError> {
        match self {
            JobCompletion::Sync(tx) => tx.send(status).map_err(|_| MbError::from_kind(MbMgrErrorKind::ChannelClosed)),
            #[cfg(feature = "async")]
            JobCompletion::Async(tx) => tx.send(status).map_err(|_| MbError::from_kind(MbMgrErrorKind::ChannelClosed)),
        }
    }
}

#[derive(Debug)]
pub struct MbRuntime {
    mgr: MbMgr,
    job_rx: mpsc::Receiver<MbJobRequest>, 
    job_queue: VecDeque<MbJobRequest>,
    _not_thread_safe: PhantomData<Rc<()>>,
}

#[derive(Debug)]
pub struct MbRuntimeHandle {
    join_handle: thread::JoinHandle<Result<(), MbError>>,
    job_tx: mpsc::Sender<MbJobRequest>,
}

#[derive(Debug)]
pub struct MbRuntimeInit {
    job_rx: mpsc::Receiver<MbJobRequest>,
    capacity: usize,
}

pub(crate) struct MbJobRequest {
    pub handle: Option<MbJobHandle<'static>>,
    pub operation: Box<dyn Operation<'static> + Send>,
    pub completion: JobCompletion,
}

// SAFETY: MbJobRequest is safe to send between threads, until handle is None
unsafe impl Send for MbJobRequest {}

impl std::fmt::Debug for MbJobRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MbJobRequest")
            .field("operation", &"<Operation>")
            .field("completion", &self.completion)
            .finish()
    }
}

impl MbRuntime {
    fn run_loop(&mut self) -> Result<(), MbError> {
        loop {
            let job_request = self.job_rx.recv();
            match job_request {
                Ok(mb_job_request) => {
                    self.process_job(mb_job_request)?;
                }
                Err(_) => return Err(MbError::from_kind(MbMgrErrorKind::ChannelClosed)),
            }
        }
    }

    fn process_job(&mut self, mut job_request: MbJobRequest) -> Result<(), MbError> {
        let (handle, completion_count) = self.mgr.handoff_job(&mut *job_request.operation)?;
        job_request.handle = Some(handle);
        self.job_queue.push_back(job_request);

        for _ in 0..completion_count {
            // SAFETY: This unwrap will never fail...
            let prev_job_request = self.job_queue.pop_front().unwrap();
            prev_job_request.completion.send(JobStatus{status: ImbStatus::IMB_STATUS_COMPLETED})?;
        }


        if self.should_flush() {
            self.flush()?;
        }
  
        Ok(())
    }

    #[inline]
    fn should_flush(&self) -> bool {
        false 
    }

    fn flush(&mut self) -> Result<(), MbError> {
        unsafe {
            self.mgr.flush_job()?;
        }
        Ok(())
    }
}


impl MbRuntimeHandle {

    pub fn join(self) -> Result<(), MbError> {
        match self.join_handle.join() {
            Ok(result) => result, 
            Err(_) => Err(MbError::from_kind(MbMgrErrorKind::RuntimeError)),
        }
    }

    /// Submit a job with scoped lifetime guarantees
    ///
    /// This method blocks until the job completes, ensuring that any
    /// borrowed data in the operation remains valid throughout execution.
    pub fn publish_job<'scope>(
        &self,
        operation: impl Operation<'scope> + Send + 'scope,
    ) -> Result<JobStatus, MbError> {
        let (completion_tx, completion_rx) = mpsc::sync_channel(1);

        let boxed_operation: Box<dyn Operation<'scope> + Send + 'scope> = Box::new(operation);

        // Transmute to 'static for storage
        // SAFETY: This is safe because:
        // 1. We block on completion_rx.recv() below
        // 2. The operation completes before this function returns
        // 3. Therefore, borrowed data ('scope) remains valid throughout
        let erased_operation: Box<dyn Operation<'static> + Send> = unsafe { std::mem::transmute(boxed_operation) };

        let request = MbJobRequest {
            handle: None,
            operation: erased_operation,
            completion: JobCompletion::Sync(completion_tx),
        };

        self.job_tx
            .send(request)
            .map_err(|_| MbError::from_kind(MbMgrErrorKind::ChannelClosed))?;

        // CRITICAL: Blocking here enforces the lifetime guarantee
        completion_rx
            .recv()
            .map_err(|_| MbError::from_kind(MbMgrErrorKind::CompletionFailed))
    }


     /// Submit a job with async/await support (only available with "async" feature)
     #[cfg(feature = "async")]
     pub async fn publish_job_async<'scope>(
         &self,
         operation: impl Operation<'scope> + Send + 'scope,
     ) -> Result<JobStatus, MbError> {
         let (completion_tx, completion_rx) = oneshot::channel();
 
         let boxed_operation: Box<dyn Operation<'scope> + Send + 'scope> = Box::new(operation);
         let erased_operation: Box<dyn Operation<'static> + Send> = unsafe { std::mem::transmute(boxed_operation) };
 
         let request = MbJobRequest {
             handle: None,
             operation: erased_operation,
             completion: JobCompletion::Async(completion_tx),
         };
 
         self.job_tx
             .send(request)
             .map_err(|_| MbError::from_kind(MbMgrErrorKind::ChannelClosed))?;
 
         completion_rx
             .await
             .map_err(|_| MbError::from_kind(MbMgrErrorKind::CompletionFailed))
     }
}

impl MbRuntimeInit {
    pub fn run(self) -> Result<(), MbError> {
        let mgr = MbMgr::new()?;

        let mut runtime = MbRuntime {
            mgr,
            job_rx: self.job_rx,
            job_queue: VecDeque::with_capacity(self.capacity),
            _not_thread_safe: PhantomData,
        };

        runtime.run_loop()?;
        Ok(())
    }
}

pub fn spawn_runtime() -> Result<MbRuntimeHandle, MbError> {
    spawn_runtime_with_capacity(128)
}

pub fn spawn_runtime_with_capacity(capacity: usize) -> Result<MbRuntimeHandle, MbError> {
    let (job_tx, job_rx) = mpsc::channel();

    let init = MbRuntimeInit { job_rx, capacity };

    let join_handle = std::thread::spawn(move || {
        init.run()
    });

    Ok(MbRuntimeHandle { job_tx, join_handle: join_handle })
}
