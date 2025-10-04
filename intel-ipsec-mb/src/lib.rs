pub mod util;
pub mod mgr;
pub mod error;
pub mod config;
pub mod job;
pub mod runtime;

pub mod hash;

// pub enum Operations {
//     Sha1((buffer: impl AsRef<[u8]>, output: impl AsMut<[u8]>)),
//     Sha256
// }


//Todo: fix all as u64 as u32 as this and as that
//Todo: fix function pointer unwraps
//Todo: add windows and mac os lib
//Todo: in bindgen.rs all function pointers are option<T> so we need to handle that
//Todo: add cpu specific function call as per cargo build flags, e.g. if avx512f is enabled, then use the avx512f function call
// currently we are using the runtime detection which is not the best option
// possible implementation would to use const trait or something
// Todo: fix all import in this crate, currently we are importing "*" everywhere for ease
// remove Copy from IMB_MGR and IMB_JOB since they should be only on heap
// Todo: fix visibility modifiers everywhere
//Todo: think about the lifetime of output and buffer, as the slice will process later in time.
//Todo: combine get_next_job, fill job and submit job
//Todo: for advanced usage give above todo func as unsafe funcs