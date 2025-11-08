use crate::error::MbError;
use crate::error::MbMgrErrorKind;
use crate::mgr::MbMgr;
use crate::operation::Operation;
use intel_ipsec_mb_sys::ImbJob;
use intel_ipsec_mb_sys::ImbMgr;
use intel_ipsec_mb_sys::ImbStatus;
use intel_ipsec_mb_sys::imb_set_session;
use std::future::Future;
use std::marker::PhantomData;
use std::mem;
use std::pin::Pin;
use std::ptr::NonNull;
use std::task::Context;
use std::task::Poll;

#[derive(Debug)]
pub struct MbJob(pub Option<NonNull<ImbJob>>);

impl MbJob {
    pub fn as_ptr(&self) -> *mut ImbJob {
        // SAFETY: as_ptr should only be called when the job is not null,
        // if the user is calling this on None, it will panic
        self.0.unwrap().as_ptr()
    }
}

#[derive(Debug)]
pub struct MbJobHandle<'anchor> {
    job: MbJob,
    _anchor: PhantomData<&'anchor ()>,
}

impl<'anchor> MbJobHandle<'anchor> {
    // SAFETY: this function is made unsafe, just to give a heads up
    // that get_job_status might return the status of the other job
    // when the same job object is re-used by the intel-ipsec-mb library
    pub unsafe fn get_job_status(&self) -> Result<JobStatus, MbError> {
        let status = unsafe { (*self.job.as_ptr()).status };
        Ok(JobStatus { status })
    }
}

impl<'anchor> Drop for MbJobHandle<'anchor> {
    fn drop(&mut self) {
        if let Ok(job_status) = unsafe { self.get_job_status() } {
            if job_status.status == ImbStatus::IMB_STATUS_COMPLETED {
                return;
            }
        }
        panic!(
            "Undefined behaviour: MbJobHandle was dropped before being properly completed! Dropping an unresolved JobHandle will cause undefined behaviour. JobHandle should be alive until the job is completed."
        );
    }
}

#[derive(Debug, Copy, Clone)]
pub struct JobStatus {
    pub status: ImbStatus,
}

pub struct BurstSession<const N: usize>([MbJob; N]);

impl<const N: usize> BurstSession<N> {
    pub fn new() -> Self {
        Self(std::array::from_fn(|_| MbJob(None)))
    }

    pub unsafe fn get_next_burst(&mut self, mb_mgr: &MbMgr, count: usize) -> Result<usize, MbError> {
        unsafe { mb_mgr.get_next_burst(&mut self.0, count) }
    }

    pub unsafe fn submit_burst(&mut self, mb_mgr: &MbMgr, count: usize) -> Result<usize, MbError> {
        unsafe { mb_mgr.submit_burst(&mut self.0, count) }
    }

    pub unsafe fn submit_burst_nocheck(&mut self, mb_mgr: &MbMgr, count: usize) -> Result<usize, MbError> {
        unsafe { mb_mgr.submit_burst_nocheck(&mut self.0, count) }
    }

    pub unsafe fn flush_burst(&mut self, mb_mgr: &MbMgr, count: usize) -> Result<usize, MbError> {
        unsafe { mb_mgr.flush_burst(&mut self.0, count) }
    }

    pub fn set_session(&self, mb_mgr: &MbMgr) -> Result<u32, MbError> {
        if self.0.is_empty() {
            return Err(MbError::from_kind(MbMgrErrorKind::NullJob));
        }
        mb_mgr.set_session(&self.0[0])
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

    pub unsafe fn get_next_burst<const N: usize>(&self, jobs:&mut [MbJob; N], count: usize) -> Result<usize, MbError> {
        let mut job_ptrs: [*mut ImbJob; N] = [std::ptr::null_mut(); N];

        // SAFETY: The pointer passed to get_next_burst is assumed to be valid otherwise we
        // would not be having a MbMgr instance
        let num_jobs = self.exec(|mgr_mut_ptr| unsafe {
            let get_next_burst_fn = (*mgr_mut_ptr).get_next_burst.unwrap();
            get_next_burst_fn(mgr_mut_ptr, count as u32, job_ptrs.as_mut_ptr())
        })?;

        for i in 0..num_jobs as usize {
            //SAFETY: At this point the jobs slice would be filled by c library
            jobs[i] = MbJob(Some(unsafe { NonNull::new_unchecked(job_ptrs[i]) }));
        }

        Ok(num_jobs as usize)
    }

    pub unsafe fn submit_burst<const N: usize>(&self, jobs: &mut [MbJob; N], count: usize) -> Result<usize, MbError> {
        let mut job_ptrs: [*mut ImbJob; N] = [std::ptr::null_mut(); N];
        
        for i in 0..count {
            job_ptrs[i] = jobs[i].0.ok_or(MbError::from_kind(MbMgrErrorKind::NullJob))?.as_ptr();
        }
        
        // SAFETY: The pointer passed to submit_burst is assumed to be valid otherwise we
        // would not be having a MbMgr instance
        let num_completed_jobs = self.exec(|mgr_mut_ptr| unsafe {
            let submit_burst_fn = (*mgr_mut_ptr).submit_burst.unwrap();
            submit_burst_fn(mgr_mut_ptr, count as u32, job_ptrs.as_mut_ptr())
        })?;
        
        for i in 0..count {
            jobs[i] = if i < num_completed_jobs as usize {
                // SAFETY: C library fills job_ptrs[0..num_completed_jobs] with valid pointers
                MbJob(Some(unsafe { NonNull::new_unchecked(job_ptrs[i]) }))
            } else {
                MbJob(None)
            };
        }
        
        Ok(num_completed_jobs as usize)
    }

    pub unsafe fn submit_burst_nocheck<const N: usize>(&self, jobs: &mut [MbJob; N], count: usize) -> Result<usize, MbError> {
        let mut job_ptrs: [*mut ImbJob; N] = [std::ptr::null_mut(); N];
        
        for i in 0..count {
            //SAFETY: At this point the called should have checked the job is not None
            job_ptrs[i] = unsafe {jobs[i].0.unwrap_unchecked().as_ptr()};
        }
        
        // SAFETY: The pointer passed to submit_burst is assumed to be valid otherwise we
        // would not be having a MbMgr instance
        let num_completed_jobs = self.exec(|mgr_mut_ptr| unsafe {
            let submit_burst_nocheck_fn = (*mgr_mut_ptr).submit_burst_nocheck.unwrap();
            submit_burst_nocheck_fn(mgr_mut_ptr, count as u32, job_ptrs.as_mut_ptr())
        })?;
        
        for i in 0..count {
            jobs[i] = if i < num_completed_jobs as usize {
                // SAFETY: C library fills job_ptrs[0..num_completed_jobs] with valid pointers
                MbJob(Some(unsafe { NonNull::new_unchecked(job_ptrs[i]) }))
            } else {
                MbJob(None)
            };
        }
        
        Ok(num_completed_jobs as usize)
    }

    pub unsafe fn flush_burst<const N: usize>(&self, jobs: &mut [MbJob; N], count: usize) -> Result<usize, MbError> {
        let mut job_ptrs: [*mut ImbJob; N] = [std::ptr::null_mut(); N];
    
        // SAFETY: The pointer passed to flush_burst is assumed to be valid otherwise we
        // would not be having a MbMgr instance
        let num_flushed_jobs = self.exec(|mgr_mut_ptr| unsafe {
            let flush_burst_fn = (*mgr_mut_ptr).flush_burst.unwrap();
            flush_burst_fn(mgr_mut_ptr, count as u32, job_ptrs.as_mut_ptr())
        })?;
        
        for i in 0..N {
            jobs[i] = if i < num_flushed_jobs as usize {
                // SAFETY: C library fills job_ptrs[0..num_flushed_jobs] with valid pointers
                MbJob(Some(unsafe { NonNull::new_unchecked(job_ptrs[i]) }))
            } else {
                MbJob(None)
            };
        }
        
        Ok(num_flushed_jobs as usize)
    }

    pub fn set_session(&self, job: &MbJob) -> Result<u32, MbError> {
        // SAFETY: The pointer passed to set_session is assumed to be valid otherwise we
        // would not be having a MbMgr instance
        let session_id = self.exec(|mgr_mut_ptr| unsafe {
           imb_set_session(mgr_mut_ptr, job.as_ptr()) as u32
        })?;

        Ok(session_id)
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
        let mut job = unsafe { self.get_next_job()? };
        let mut completion_count = 0;

        if job.0.is_none() {
            // Think: Should we flush all the jobs or just create space for 1 job?
            completion_count += self.force_complete_job()?;
            job = unsafe { self.get_next_job()? };
            if job.0.is_none() {
                return Err(MbError::from_kind(MbMgrErrorKind::NoJobAvailable));
            }
        }

        operation.fill_job(&job, &self)?;
        let completed_from_submit = unsafe { self.submit_job()? };

        if completed_from_submit.0.is_some() {
            completion_count += 1;
        }

        loop {
            match unsafe { self.get_completed_job()? }.0 {
                Some(_) => completion_count += 1,
                None => break,
            }
        }

        Ok((
            MbJobHandle {
                job: job,
                _anchor: PhantomData,
            },
            completion_count,
        ))
    }

    pub fn force_complete_job(&self) -> Result<usize, MbError> {
        let mut completion_count = 0;
        
        // Just loop flush_job until NULL
        loop {
            match unsafe { self.flush_job()? }.0 {
                Some(_) => completion_count += 1,
                None => break,  // No more jobs to flush
            }
        }
        
        Ok(completion_count)
    }

    pub fn handoff_job_burst<'anchor, T, const N: usize>(
        &self,
        burst_session: &mut BurstSession<N>,
        operations: &mut [T; N],
    ) -> Result<([MbJobHandle<'anchor>; N], usize, usize), MbError>
    where
        T: Operation<'anchor>,
    {
        let mut num_jobs = unsafe { burst_session.get_next_burst(self, N)? };
        let mut completion_count = 0;
        if num_jobs == 0 {
            completion_count += self.force_complete_job_burst(burst_session)?;
            num_jobs = unsafe { burst_session.get_next_burst(self, N)? };
            if num_jobs == 0 {
                return Err(MbError::from_kind(MbMgrErrorKind::NoJobAvailable));
            }
        }

        for i in 0..num_jobs {
            operations[i].fill_job(&burst_session.0[i], &self)?;
        }

        let completed_from_submit = unsafe { burst_session.submit_burst(self, N)? };
        completion_count += completed_from_submit as usize;

        let mut job_handles: [MbJobHandle<'anchor>; N] = std::array::from_fn(|_| MbJobHandle { job: MbJob(None), _anchor: PhantomData });


        for i in 0..num_jobs {
            job_handles[i].job = MbJob(Some(unsafe { NonNull::new_unchecked(burst_session.0[i].0.unwrap().as_ptr()) }));
        }

        Ok((job_handles, num_jobs, completion_count))
    }

    pub fn force_complete_job_burst<const N: usize>(&self, burst_session: &mut BurstSession<N>) -> Result<usize, MbError> {
        let completion_count = unsafe { burst_session.flush_burst(self, N)? };
        Ok(completion_count as usize)
    }
}
