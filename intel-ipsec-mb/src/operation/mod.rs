pub mod hash;
pub mod cipher;

use crate::error::MbError;
use crate::job::MbJob;
use crate::mgr::MbMgr;


pub trait Operation<'anchor> 
{
    fn fill_job(&mut self, job: &MbJob, mgr: &MbMgr) -> Result<&'anchor (), MbError>;
}
