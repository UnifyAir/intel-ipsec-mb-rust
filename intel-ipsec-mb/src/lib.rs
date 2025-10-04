pub mod util;
pub mod mgr;
pub mod error;
pub mod config;
pub mod job;
pub mod runtime;

pub mod hash;


//Todo: fix all as u64 as u32 as this and as that
//Todo: fix function pointer unwraps
//Todo: add windows and mac os lib
//Todo: in bindgen.rs all function pointers are option<T> so we need to handle that
//Todo: add cpu specific function call as per cargo build flags, e.g. if avx512f is enabled, then use the avx512f function call
// currently we are using the runtime detection which is not the best option
// possible implementation would to use const trait or something
// Todo: fix all import in this crate, currently we are importing "*" everywhere for ease
// remove Copy from IMB_MGR and IMB_JOB since they should be only on heap
