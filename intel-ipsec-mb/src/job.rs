use crate::error::MbError;
use crate::mgr::MbMgr;
use intel_ipsec_mb_sys::ImbJob;
use std::ptr::NonNull;
use std::marker::PhantomData;
use intel_ipsec_mb_sys::ImbMgr;
use intel_ipsec_mb_sys::ImbStatus;


// Todo: remove this non null as soon as possible, MbJob will be null when return via get_completed_job or submit_job
pub struct MbJob(pub Option<NonNull<ImbJob>>);

impl MbJob {

    pub fn as_ptr(&self) -> *const ImbJob {
        // SAFETY: as_ptr should only be called when the job is not null,
        // if the user is calling this on None, it will panic
        self.0.unwrap().as_ptr()
    }

    pub fn as_mut_ptr(&mut self) -> *mut ImbJob {
        // SAFETY: as_ptr should only be called when the job is not null,
        // if the user is calling this on None, it will panic
        self.0.unwrap().as_ptr()
    }

    // pub fn get_status(&self) -> Result<u32, MbMgrError> {
    //     // SAFETY: The pointer passed to get_status is assumed to be valid otherwise we
    //     // would not be having a MbJob instance
    //     let status = self.exec(|job_mut_ptr| unsafe {
    //         let get_status_fn = (*job_mut_ptr).get_status.unwrap();
    //         get_status_fn(job_mut_ptr)
    //     })?;
}



impl MbMgr {
    pub fn get_next_job(&self) -> Result<MbJob, MbError> {
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

    pub fn submit_job(&self, job: &MbJob) -> Result<(MbJobGuard<'_, '_, '_>, Option<MbJob>), MbError> {
        debug_assert!(job.0.is_some(), "MbJob passed to submit_job must not be None");
        //SAFETY: The job is not Option::None, if it is null, the user should not be calling this function
        let submit_job_guard = MbJobGuard {
            job: job.0.unwrap(),
            _manager: PhantomData,
            _input: PhantomData,
            _output: PhantomData,
        };
        // SAFETY: The pointer passed to submit_job is assumed to be valid otherwise we
        // would not be having a MbMgr instance
        let job = self.exec(|mgr_mut_ptr| unsafe {
            let submit_job_fn = (*mgr_mut_ptr).submit_job.unwrap();
            submit_job_fn(mgr_mut_ptr)
        })?;
        if job.is_null() {
            return Ok((submit_job_guard, None));
        }
        Ok((submit_job_guard, Some(MbJob(Some(unsafe { NonNull::new_unchecked(job) })))))
    }

    pub fn get_completed_job(&self) -> Result<MbJob, MbError> {
        // SAFETY: The pointer passed to get_completed_job is assumed to be valid otherwise we
        // would not be having a MbMgr instance
        let job = self.exec(|mgr_mut_ptr| unsafe {
            let get_completed_job_fn = (*mgr_mut_ptr).get_completed_job.unwrap();
            get_completed_job_fn(mgr_mut_ptr)
        })?;
        if job.is_null() {
            return Ok(MbJob(None));
        }
        Ok(MbJob(Some(unsafe { NonNull::new_unchecked(job) })))
    }

    pub fn flush_job(&self) -> Result<MbJob, MbError> {
        // SAFETY: The pointer passed to flush_job is assumed to be valid otherwise we
        // would not be having a MbMgr instance
        let job =self.exec(|mgr_mut_ptr| unsafe {
            let flush_job_fn = (*mgr_mut_ptr).flush_job.unwrap();
            flush_job_fn(mgr_mut_ptr)
        })?;
        if job.is_null() {
            return Ok(MbJob(None));
        }
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

}


#[must_use = "Job must be completed"]
pub struct MbJobGuard<'mgr, 'buf, 'out> {
    job: NonNull<ImbJob>,
    _manager: PhantomData<&'mgr ImbMgr>,
    _input: PhantomData<&'buf [u8]>,
    _output: PhantomData<&'out mut [u8]>,
}

impl<'mgr, 'buf, 'out> MbJobGuard<'mgr, 'buf, 'out> {
    pub fn status(&self) -> Result<ImbStatus, MbError> {
        // SAFETY: The job is not Option::None, if it is null, the user should not be calling this function
        let status = unsafe { (*self.job.as_ptr()).status };
        Ok(status)
    }
    
}

impl Drop for MbJobGuard<'_, '_, '_> {
    fn drop(&mut self) {
        panic!(
            "MbJobGuard dropped before completion!\n\
             This would cause use-after-free. You must call try_complete() or wait_complete()."
        );
    }
}