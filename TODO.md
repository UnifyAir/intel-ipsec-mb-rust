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
// Todo: fix naming of mb_mgr and mgr, there is some inconsistency
// Todo: add derive debug, copy and suitable traits where needed
// Todo: to reduce runtime ovehead, add bdebuf assertions
// Todo: why the below need mut output
//pub fn fill_job_sha1(
    // &self,
    // job: &mut MbJob,
    // buffer: impl AsRef<[u8]>,
    // mut output: impl AsMut<[u8]>,

// Todo: reformat the whole everything is super messed up    
//Todo: few days back I changed the MbMgr from "to_mut_ptr" and "to_ptr" to just "to_ptr" removed the
// mut vairiant look out in the future was thre previous design was good
//Todo: remove all unwraps and use proper error handling
/**
* get_next_job returns a job object. This must be filled in and returned
* via submit_job before get_next_job is called again.
* After submit_job is called, one should call get_completed_job() at least
* once (and preferably until it returns NULL).
* get_completed_job and flush_job returns a job object. This job object ceases to be usable at the next call to get_next_job 
*/
