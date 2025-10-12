use crate::error::{MbError, MbMgrErrorKind};
use crate::job::MbJob;
use crate::mgr::MbMgr;
use crate::operation::Operation;
use intel_ipsec_mb_sys::*;


#[derive(Debug)]
pub struct Snow3gUea2<'buf, 'out, B: AsRef<[u8]> + ?Sized + 'buf, O: AsMut<[u8]> + ?Sized + 'out> {
    pub buffer: &'buf B,
    pub output: &'out mut O,
}


impl<'anchor, 'buf, 'out, B, O> Operation<'anchor> 
    for Snow3gUea2<'buf, 'out, B, O>
where
    'buf: 'anchor, 
    'out: 'anchor, 
    B: AsRef<[u8]> + ?Sized + 'buf,
    O: AsMut<[u8]> + ?Sized + 'out,
{
    fn fill_job(&mut self, _job: &MbJob, _mgr: &MbMgr) -> Result<&'anchor (), MbError> {
        todo!()
    }
}
