use crate::error::MbError;
use crate::error::MbMgrErrorKind;
use crate::mgr::MbMgr;
use intel_ipsec_mb_sys::ImbJob;
use intel_ipsec_mb_sys::ImbMgr;
use intel_ipsec_mb_sys::ImbStatus;
use std::mem;
use std::future::Future;
use std::marker::PhantomData;
use std::pin::Pin;
use std::ptr::NonNull;
use std::task::Context;
use std::task::Poll;
use crate::operation::Operation;

// Todo: remove this non null as soon as possible, MbJob will be null when return via get_completed_job or submit_job

#[derive(Eq, Hash, PartialEq)]
pub struct MbJob(pub Option<NonNull<ImbJob>>);

pub struct MbJobHandle<'anchor> {
    job_id: *const ImbJob,
    _anchor: PhantomData<&'anchor ()>,
}

//SAFETY: we are just using *const ImbJob, as an identifier for the job, not for the job itself
unsafe impl Send for MbJobHandle<'static> {}

impl<'anchor> MbJobHandle<'anchor> {
    pub fn resolve(self) -> Result<(), MbError> {
        mem::forget(self);
        Ok(())
    }

    pub fn get_job_status(&self) -> Result<JobStatus, MbError> {
        let status = unsafe { (*self.job_id).status };
        Ok(JobStatus { status })
    }
}

impl<'anchor> Drop for MbJobHandle<'anchor> {
    fn drop(&mut self) {
        panic!("Undefined behaviour: MbJobHandle was dropped before being properly completed! Dropping an unresolved JobHandle will cause undefined behaviour. You must consume a JobHandle by calling its completion methods (e.g., wait, complete, or poll).");
    }
}

#[derive(Debug, Copy, Clone)]
pub struct JobStatus {
    pub status: ImbStatus,
}


impl MbJob {
      pub fn as_ptr(&self) -> *mut ImbJob {
        // SAFETY: as_ptr should only be called when the job is not null,
        // if the user is calling this on None, it will panic
        self.0.unwrap().as_ptr()
    }
}

impl MbMgr {
        pub unsafe fn get_next_job(&self) -> Result<MbJob, MbError> {
        // SAFETY: The pointer passed to get_next_job is assumed to be valid otherwise we
        // would not be having a MbMgr instance
        let job = self.exec(|mgr_mut_ptr| unsafe {
            let get_next_job_fn = (*mgr_mut_ptr).get_next_job.unwrap();
            get_next_job_fn(mgr_mut_ptr)
        })?;
        //Todo: This situation should not happen, will remove this in the future
        if job.is_null() {
            return Ok(MbJob(None));
        }
        Ok(MbJob(Some(unsafe { NonNull::new_unchecked(job) })))
    }

    pub unsafe fn submit_job(&self) -> Result<MbJob, MbError> {
        // SAFETY: The pointer passed to submit_job is assumed to be valid otherwise we
        // would not be having a MbMgr instance
        let job = self.exec(|mgr_mut_ptr| unsafe {
            let submit_job_fn = (*mgr_mut_ptr).submit_job.unwrap();
            submit_job_fn(mgr_mut_ptr)
        })?;
        if job.is_null() {
            return Ok(MbJob(None));
        }
        //SAFETY: At this point the job is not null
        Ok(MbJob(Some(unsafe { NonNull::new_unchecked(job) })))
    }

    pub unsafe fn submit_job_nocheck(&self) -> Result<MbJob, MbError> {
        // SAFETY: The pointer passed to submit_job_nocheck is assumed to be valid otherwise we
        // would not be having a MbMgr instance
        let job = self.exec(|mgr_mut_ptr| unsafe {
            let submit_job_nocheck_fn = (*mgr_mut_ptr).submit_job_nocheck.unwrap();
            submit_job_nocheck_fn(mgr_mut_ptr)
        })?;
        if job.is_null() {
            return Ok(MbJob(None));
        }
        //SAFETY: At this point the job is not null
        Ok(MbJob(Some(unsafe { NonNull::new_unchecked(job) })))
    }

    pub unsafe fn get_completed_job(&self) -> Result<MbJob, MbError> {
        // SAFETY: The pointer passed to get_completed_job is assumed to be valid otherwise we
        // would not be having a MbMgr instance
        let job = self.exec(|mgr_mut_ptr| unsafe {
            let get_completed_job_fn = (*mgr_mut_ptr).get_completed_job.unwrap();
            get_completed_job_fn(mgr_mut_ptr)
        })?;
        if job.is_null() {
            return Ok(MbJob(None));
        }
        //SAFETY: At this point the job is not null
        Ok(MbJob(Some(unsafe { NonNull::new_unchecked(job) })))
    }

    pub unsafe fn flush_job(&self) -> Result<MbJob, MbError> {
        // SAFETY: The pointer passed to flush_job is assumed to be valid otherwise we
        // would not be having a MbMgr instance
        let job = self.exec(|mgr_mut_ptr| unsafe {
            let flush_job_fn = (*mgr_mut_ptr).flush_job.unwrap();
            flush_job_fn(mgr_mut_ptr)
        })?;
        if job.is_null() {
            return Ok(MbJob(None));
        }
        //SAFETY: At this point the job is not null
        Ok(MbJob(Some(unsafe { NonNull::new_unchecked(job) })))
    }

    pub fn queue_size(&self) -> Result<u32, MbError> {
        // SAFETY: The pointer passed to queue_size is assumed to be valid otherwise we
        // would not be having a MbMgr instance
        let size = self.exec(|mgr_mut_ptr| unsafe {
            let queue_size_fn = (*mgr_mut_ptr).queue_size.unwrap();
            queue_size_fn(mgr_mut_ptr)
        })?;
        Ok(size)
    }

    pub fn handoff_job<'anchor, T>(
        &self,
        operation: &mut T,
    ) -> Result<(MbJobHandle<'anchor>, bool), MbError>
    where
        T: Operation<'anchor> + ?Sized,
    {
        let job = unsafe { self.get_next_job()? };
        // Todo: use some queue function from intel ipsec library, or think about it.
        if job.0.is_none() {
            return Err(MbError::from_kind(MbMgrErrorKind::NoJobAvailable));
        }

        // SAFETY: We're extending the lifetime of the job reference to 'anchor.
        // This is safe because:
        // 1. The underlying C pointer from IMB_GET_NEXT_JOB remains valid until
        //    the job is submitted/flushed or the manager is destroyed
        // 2. The Operation<'anchor> trait bound ensures that input buffers ('buf: 'anchor)
        //    and output buffers ('out: 'anchor) remain valid for the 'anchor lifetime
        // 3. We leak the MbJob wrapper (via mem::forget) so its Drop doesn't run
        // 4. The caller MUST ensure they call flush_job() or submit_job() before
        //    dropping the returned reference to complete the job lifecycle
        // let job_ref: &'anchor MbJob = unsafe { std::mem::transmute::<&MbJob, &'anchor MbJob>(&job) };

        operation.fill_job(&job, &self)?;

        let completed_job = unsafe { self.submit_job()? };
        let is_previous_job_finished = completed_job.0.is_some();

        // Leak the job wrapper so it's not dropped
        // The underlying C pointer is managed by the Intel IPSec library
        // mem::forget(job);
        Ok((MbJobHandle {
            job_id: job.0.unwrap().as_ptr() as *const ImbJob,
            _anchor: PhantomData,
        }, is_previous_job_finished))
    }

}
