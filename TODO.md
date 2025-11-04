# TODO

## Type Safety & Casting
- [ ] Fix all `as u64`, `as u32`, and other type casts throughout the codebase
- [ ] Use `std::ffi::c_void` everywhere instead of `*mut c_void` or similar

## Error Handling
- [ ] Remove all `unwrap()` calls and implement proper error handling
- [ ] Fix function pointer unwraps

## Function Pointers & Bindgen
- [ ] In `bindgen.rs`, all function pointers are `Option<T>` - handle this properly
- [ ] Fix function pointer handling throughout the codebase

## Platform Support
- [ ] Think about adding Windows support (Extremely low priority)

## Performance & CPU Features
- [ ] Add CPU-specific function calls based on cargo build flags (e.g., if `avx512f` is enabled, use the AVX512F function call)
  - Currently using runtime detection which is not optimal
  - Possible implementation: use const traits or similar approach

## Code Organization
- [ ] Fix all imports in this crate - currently using `*` everywhere for ease
- [ ] Fix visibility modifiers everywhere
- [ ] Reformat the entire codebase - everything is super messed up

## API Design
- [ ] Remove `Copy` from `IMB_MGR` and `IMB_JOB` since they should only be on the heap
- [ ] Combine `get_next_job`, `fill_job`, and `submit_job` functions
- [ ] For advanced usage, provide the above combined functions as unsafe functions
- [ ] Fix naming inconsistency between `mb_mgr` and `mgr`
- [ ] Add proper graceful runtime shutdown function

## Traits & Debugging
- [ ] Add derive `Debug`, `Copy`, and other suitable traits where needed

## Memory & Lifetimes
- [ ] Think about the lifetime of output and buffer - slices will process later in time
- [ ] Add debug assertions to reduce runtime overhead

## Documentation
- [ ] Investigate why `fill_job_sha1` needs `mut output`:
  ```rust
  pub fn fill_job_sha1(
      &self,
      job: &mut MbJob,
      buffer: impl AsRef<[u8]>,
      mut output: impl AsMut<[u8]>,
  ```

## Job Management Notes
> **Important:** `get_next_job` returns a job object. This must be filled in and returned via `submit_job` before `get_next_job` is called again. After `submit_job` is called, one should call `get_completed_job()` at least once (and preferably until it returns NULL). `get_completed_job` and `flush_job` return a job object. This job object ceases to be usable at the next call to `get_next_job`.

## Design Considerations
- [ ] Review: A few days ago, `MbMgr` was changed from having both `to_mut_ptr()` and `to_ptr()` to just `to_ptr()` (removed the mut variant). Look out in the future - was the previous design better?
