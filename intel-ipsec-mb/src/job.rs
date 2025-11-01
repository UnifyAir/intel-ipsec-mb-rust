use crate::error::MbError;
use crate::error::MbMgrErrorKind;
use crate::mgr::MbMgr;
use crate::operation::Operation;
use intel_ipsec_mb_sys::ImbJob;
use intel_ipsec_mb_sys::ImbMgr;
use intel_ipsec_mb_sys::ImbStatus;
use std::future::Future;
use std::marker::PhantomData;
use std::mem;
use std::pin::Pin;
use std::ptr::NonNull;
use std::task::Context;
use std::task::Poll;

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
        panic!(
            "Undefined behaviour: MbJobHandle was dropped before being properly completed! Dropping an unresolved JobHandle will cause undefined behaviour. You must consume a JobHandle by calling its completion methods (e.g., wait, complete, or poll)."
        );
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
    ) -> Result<(MbJobHandle<'anchor>, usize), MbError>
    where
        T: Operation<'anchor> + ?Sized,
    {
         // SAFETY CHECK: Prevent UB from reused job pointers
         if self.undrained_completion_count.get() > 0 {
            return Err(MbError::from_kind(
                MbMgrErrorKind::UndrainedCompletions
            ));
        }

        let job = unsafe { self.get_next_job()? };

        if job.0.is_none() {
            return Err(MbError::from_kind(MbMgrErrorKind::NoJobAvailable));
        }

        operation.fill_job(&job, &self)?;
        let completed_from_submit = unsafe { self.submit_job()? };

        let mut completion_count = 0;

        if completed_from_submit.0.is_some() {
            completion_count += 1;
        }

        // Drain get_completed_job (required by C API)
        loop {
            match unsafe { self.get_completed_job()? }.0 {
                Some(_) => completion_count += 1,
                None => break,
            }
        }

        // Store count for safety check
        if completion_count > 0 {
            self.undrained_completion_count.set(completion_count);
        }

        Ok((
            MbJobHandle {
                job_id: job.0.unwrap().as_ptr() as *const ImbJob,
                _anchor: PhantomData,
            },
            completion_count,
        ))
    }

    pub fn force_completion(&self) -> Result<usize, MbError> {
        let mut completion_count = 0;
        
        // Just loop flush_job until NULL
        loop {
            match unsafe { self.flush_job()? }.0 {
                Some(_) => completion_count += 1,
                None => break,  // No more jobs to flush
            }
        }
        
        // Mark that completions need handling if any completed
        if completion_count > 0 {
            self.undrained_completion_count.set(completion_count);
        }
        
        Ok(completion_count)
    }
}
