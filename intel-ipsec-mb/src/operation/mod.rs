pub mod hash;
pub mod cipher;

use crate::error::MbError;
use crate::job::MbJob;


pub trait Operation<'anchor> 
{
    fn fill_job(&mut self, job: &MbJob) -> Result<&'anchor (), MbError>;
}
