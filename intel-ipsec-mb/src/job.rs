use crate::error::MbMgrError;
use crate::mgr::MbMgr;
use intel_ipsec_mb_sys::ImbJob;
use std::ptr::NonNull;


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

pub struct MbJobHandle<'buf, 'out, F>
where
    F: FnOnce,
{
    callback: Option<F>,
    job: NonNull<ImbJob>,
    _input_lifetime: PhantomData<&'buf [u8]>,
    _output_lifetime: PhantomData<&'out mut [u8]>,
}

impl MbMgr {
    pub fn get_next_job(&mut self) -> Result<MbJob, MbMgrError> {
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

    pub fn submit_job(&mut self) -> Result<MbJob, MbMgrError> {
        // SAFETY: The pointer passed to submit_job is assumed to be valid otherwise we
        // would not be having a MbMgr instance
        let job = self.exec(|mgr_mut_ptr| unsafe {
            let submit_job_fn = (*mgr_mut_ptr).submit_job.unwrap();
            submit_job_fn(mgr_mut_ptr)
        })?;
        if job.is_null() {
            return Ok(MbJob(None));
        }
        Ok(MbJob(Some(unsafe { NonNull::new_unchecked(job) })))
    }

    pub fn get_completed_job(&mut self) -> Result<MbJob, MbMgrError> {
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

    pub fn flush_job(&mut self) -> Result<MbJob, MbMgrError> {
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

    pub fn queue_size(&mut self) -> Result<u32, MbMgrError> {
        // SAFETY: The pointer passed to queue_size is assumed to be valid otherwise we
        // would not be having a MbMgr instance
        let size = self.exec(|mgr_mut_ptr| unsafe {
            let queue_size_fn = (*mgr_mut_ptr).queue_size.unwrap();
            queue_size_fn(mgr_mut_ptr)
        })?;
        Ok(size)
    }

}

