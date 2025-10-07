use crate::config::MbMgrConfig;
use crate::error::MbError;
use crate::job::JobStatus;
use crate::job::MbJob;
use intel_ipsec_mb_sys::{ImbMaxJobs, ImbMgr, alloc_mb_mgr, free_mb_mgr, init_mb_mgr_auto};
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::ptr::NonNull;
use std::task::Waker;
use intel_ipsec_mb_sys::ImbStatus;
use intel_ipsec_mb_sys::IMB_JOB;
use std::rc::Rc;
use std::array;

use std::marker::PhantomData;

pub struct MbMgr {
    mgr: NonNull<ImbMgr>,
    // pub(crate) outstanding_jobs: RefCell<HashMap<*const IMB_JOB, JobStatus>>,
    // pub(crate) completed_jobs: RefCell<HashMap<*const IMB_JOB, JobStatus>>,
    // pub(crate) wakers: RefCell<HashMap<*const IMB_JOB, Waker>>,
    _not_thread_safe: PhantomData<Rc<()>>,
}

impl MbMgr {
    // For operations that don't mutate (reading state, etc.)
    pub fn as_ptr(&self) -> *mut ImbMgr {
        self.mgr.as_ptr()
    }

    // Temporary disabled as we are not using it, it was there for
    // safety, but it is not needed
    // pub fn as_mut_ptr(&mut self) -> *mut ImbMgr {
    //     self.mgr.as_ptr()
    // }
}

//Todo fix this
impl fmt::Debug for MbMgr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        unsafe {
            let mgr_ref = self.mgr.as_ref();
            f.debug_struct("MbMgr")
                .field("flags", &format!("0x{:x}", mgr_ref.flags))
                .field("features", &format!("0x{:x}", mgr_ref.features))
                .field("used_arch_type", &mgr_ref.used_arch_type)
                .field("used_arch", &mgr_ref.used_arch)
                .field("imb_errno", &mgr_ref.imb_errno)
                .finish()
        }
    }
}

impl Drop for MbMgr {
    fn drop(&mut self) {
        // Todo: can we tackle this with lifetime..., such that jobhandle should not outlive the manager
        // if !self.outstanding_jobs.borrow().is_empty() {
        //     panic!(
        //         "ImbMgr dropped with {} outstanding jobs!",
        //         self.outstanding_jobs.borrow().len()
        //     );
        // }
        unsafe {
            free_mb_mgr(self.mgr.as_ptr());
        }
    }
}

impl MbMgr {
    pub fn new() -> Result<Self, MbError> {
        Self::with_config(MbMgrConfig::default())
    }

    pub fn with_config(config: MbMgrConfig) -> Result<Self, MbError> {
        unsafe {
            let mgr = alloc_mb_mgr(config.to_flags());

            if let Some(err) = MbError::capture_global() {
                return Err(err);
            }

            let mgr = NonNull::new_unchecked(mgr);
            let mut manager = Self {
                mgr,
                // outstanding_jobs: RefCell::new(HashMap::new()),
                // completed_jobs: RefCell::new(HashMap::new()),
                // wakers: RefCell::new(HashMap::new()),
                _not_thread_safe: PhantomData,
            };

            Self::exec(&mut manager, |mgr| {
                init_mb_mgr_auto(mgr, std::ptr::null_mut())
            })?;

            Ok(manager)
        }
    }

    pub fn builder() -> MbMgrConfig {
        MbMgrConfig::new()
    }

    pub(crate) fn exec<F, R>(&self, f: F) -> Result<R, MbError>
    where
        F: FnOnce(*mut ImbMgr) -> R,
    {
        let result = f(self.as_ptr());

        match MbError::capture(self) {
            Some(err) => Err(err),
            None => Ok(result),
        }
    }
}
