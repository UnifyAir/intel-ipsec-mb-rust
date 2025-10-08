use intel_ipsec_mb_sys::{IMB_FLAG_SHANI_OFF, IMB_FLAG_GFNI_OFF};
use intel_ipsec_mb_sys::ImbArch;
use crate::mgr::MbMgr;
use crate::error::MbError;


#[derive(Debug, Clone)]
pub struct MbRuntimeConfig {
    quick_start: bool,
}

impl Default for MbRuntimeConfig {
    fn default() -> Self {
        Self {
            quick_start: false,
        }
    }
}

impl MbRuntimeConfig {
    pub fn new() -> Self {
        Self::default()
    }
}


/// Configuration for Multi-Buffer Manager
#[derive(Debug, Clone)]
pub struct MbMgrConfig {
    _arch_mode: ArchMode,
    disable_shani: bool,
    disable_gfni: bool,
}

/// How to select the architecture
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ArchMode {
    /// Auto-detect: Use compile-time features if available, otherwise runtime detection
    Auto,
    /// Force runtime detection even if compile-time features are available
    ForceRuntime,
    /// Force specific architecture at runtime (will crash on incompatible CPU!)
    #[cfg(feature = "allow-forced")]
    Forced(ImbArch),
}

impl Default for MbMgrConfig {
    fn default() -> Self {
        Self {
            _arch_mode: ArchMode::Auto,  // Smart default!
            disable_shani: false,
            disable_gfni: false,
        }
    }
}

impl MbMgrConfig {
    /// Create new configuration with automatic detection (default)
    /// 
    /// This automatically detects the best architecture to use:
    /// 1. If compiled with RUSTFLAGS (e.g., `target-feature=+avx512f`), uses that
    /// 2. Otherwise, uses runtime CPU detection
    /// 
    /// This is the recommended default for most use cases.
    /// 
    /// # Example
    /// 
    /// ```
    /// use intel_ipsec_mb::MbMgrConfig;
    /// 
    /// // Simple - automatically does the right thing
    /// let mgr = MbMgrConfig::new().build()?;
    /// ```
    /// 
    /// ```bash
    /// # If you compile with RUSTFLAGS, it automatically uses them:
    /// RUSTFLAGS="-C target-feature=+avx512f" cargo build --release
    /// # Your binary will use AVX-512!
    /// 
    /// # If you compile without RUSTFLAGS:
    /// cargo build --release
    /// # Your binary will use runtime detection (safe everywhere)
    /// ```
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Force runtime CPU detection
    /// 
    /// This explicitly forces runtime CPU detection even if the binary was
    /// compiled with specific RUSTFLAGS. Useful when you want maximum
    /// compatibility and don't want to rely on compile-time features.
    /// 
    /// # Example
    /// 
    /// ```
    /// use intel_ipsec_mb::MbMgrConfig;
    /// 
    /// // Always use runtime detection, even if compiled with RUSTFLAGS
    /// let mgr = MbMgrConfig::with_runtime_detection().build()?;
    /// ```
    pub fn with_runtime_detection() -> Self {
        Self {
            _arch_mode: ArchMode::ForceRuntime,
            ..Default::default()
        }
    }
    
    /// Force specific architecture without CPU detection
    /// 
    /// **Note**: This function is only available when the `allow-forced` feature is enabled.
    /// 
    /// # Safety Considerations
    /// 
    /// This function bypasses runtime CPU feature detection. The program will
    /// crash with "Illegal instruction" (SIGILL) if the CPU does not support
    /// the specified architecture. This is not `unsafe` in Rust's memory-safety
    /// sense, but will cause immediate program termination on incompatible hardware.
    /// 
    /// This feature increases binary size as all architecture paths must be included
    /// for runtime selection. If you're forcing a specific architecture, prefer using
    /// RUSTFLAGS instead:
    /// 
    /// ```bash
    /// # Prefer this (smaller binary):
    /// RUSTFLAGS="-C target-feature=+avx512f" cargo build --release
    /// 
    /// # Over this (larger binary):
    /// cargo build --release --features allow-forced
    /// # Then: MbMgrConfig::force_architecture_unchecked(ImbArch::IMB_ARCH_AVX512)
    /// ```
    /// 
    /// Only use this when:
    /// - You need runtime configuration based on external config files
    /// - You need to switch architectures dynamically
    /// - You control the deployment environment completely
    /// 
    /// # Example
    /// 
    /// ```toml
    /// # Cargo.toml
    /// [dependencies]
    /// intel-ipsec-mb = { version = "0.1", features = ["allow-forced"] }
    /// ```
    /// 
    /// ```
    /// use intel_ipsec_mb::{MbMgrConfig, ImbArch};
    /// 
    /// // Read from config file at runtime
    /// let arch = match config.crypto_mode {
    ///     "high_performance" if is_x86_feature_detected!("avx512f") => {
    ///         ImbArch::IMB_ARCH_AVX512
    ///     }
    ///     "balanced" if is_x86_feature_detected!("avx2") => {
    ///         ImbArch::IMB_ARCH_AVX2
    ///     }
    ///     _ => {
    ///         // Fallback to safe default
    ///         return MbMgrConfig::new().build()?;
    ///     }
    /// };
    /// 
    /// let mgr = MbMgrConfig::force_architecture_unchecked(arch).build()?;
    /// 
    /// let mgr = MbMgrConfig::force_architecture_unchecked(ImbArch::IMB_ARCH_AVX512).build()?;
    /// ```
    #[cfg(feature = "allow-forced")]
    pub fn force_architecture_unchecked(arch: ImbArch) -> Self {
        Self {
            _arch_mode: ArchMode::Forced(arch),
            ..Default::default()
        }
    }
    
    /// Disable SHANI (SHA Extensions) even if available
    /// 
    /// This forces the library to use non-SHANI code paths even if the CPU
    /// supports SHA Extensions. Useful for:
    /// - Testing fallback code paths
    /// - Working around potential hardware bugs
    /// - Benchmarking performance differences
    /// 
    /// # Example
    /// 
    /// ```
    /// use intel_ipsec_mb::MbMgrConfig;
    /// 
    /// let mgr = MbMgrConfig::new()
    ///     .disable_shani()
    ///     .build()?;
    /// ```
    pub fn disable_shani(mut self) -> Self {
        self.disable_shani = true;
        self
    }
    
    /// Disable GFNI (Galois Field New Instructions) even if available
    /// 
    /// This forces the library to use non-GFNI code paths even if the CPU
    /// supports Galois Field instructions. Useful for:
    /// - Testing fallback code paths
    /// - Working around potential hardware bugs
    /// - Benchmarking performance differences
    /// 
    /// # Example
    /// 
    /// ```
    /// use intel_ipsec_mb::MbMgrConfig;
    /// 
    /// let mgr = MbMgrConfig::new()
    ///     .disable_gfni()
    ///     .build()?;
    /// ```
    pub fn disable_gfni(mut self) -> Self {
        self.disable_gfni = true;
        self
    }
    
    /// Build the Multi-Buffer Manager with this configuration
    /// 
    /// # Errors
    /// 
    /// Returns `MbMgrError::AllocationFailed` if the Intel library fails to
    /// allocate the manager structure.
    /// 
    /// # Example
    /// 
    /// ```
    /// use intel_ipsec_mb::MbMgrConfig;
    /// 
    /// let mgr = MbMgrConfig::new()
    ///     .disable_shani()
    ///     .disable_gfni()
    ///     .build()?;
    /// ```
    pub fn build(self) -> Result<MbMgr, MbError> {
        MbMgr::with_config(self)
    }
    
    /// Convert config to allocation flags for Intel library
    pub(crate) fn to_flags(&self) -> u64 {
        let mut flags = 0u32;
        
        if self.disable_shani {
            flags |= IMB_FLAG_SHANI_OFF;
        }
        if self.disable_gfni {
            flags |= IMB_FLAG_GFNI_OFF;
        }
        
        flags as u64
    }
    
    /// Determine which architecture to use based on configuration
    // Todo: Currently this function is unuseable, intel library anyway going to call detect_arch
    // You would save 2 instruction + 2 branch instructions, but the following mactch instruction
    // will balance it out, so not worth it. The initialization time would be same.
    pub(crate) fn _determine_architecture(&self) -> Option<ImbArch> {
        match self._arch_mode {
            ArchMode::Auto => {
                // Try compile-time detection first
                if let Some(arch) = Self::_detect_compile_time_architecture() {
                    return Some(arch);
                }
                // Fall back to runtime detection
                None
            }
            ArchMode::ForceRuntime => None, // Always runtime
            #[cfg(feature = "allow-forced")]
            ArchMode::Forced(arch) => Some(arch),
        }
    }
    
    /// Detect architecture from compile-time target features
    /// 
    /// Checks #[cfg(target_feature)] to determine what was specified in RUSTFLAGS.
    /// Priority order: AVX10 > AVX-512 > AVX2 > SSE
    /// 
    /// Returns None if no specific features detected (will use runtime detection)
    fn _detect_compile_time_architecture() -> Option<ImbArch> {
        // Check for AVX10 (newest)
        // Note: AVX10 detection might require checking for avx10.1-512 or similar
        // This is future-proofing for when rustc supports it
        #[cfg(target_feature = "avx10.1")]
        return Some(ImbArch::IMB_ARCH_AVX10);
        
        // Check for AVX-512
        #[cfg(all(
            target_feature = "avx512f",
            not(target_feature = "avx10.1")
        ))]
        return Some(ImbArch::IMB_ARCH_AVX512);
        
        // Check for AVX2
        #[cfg(all(
            target_feature = "avx2",
            not(target_feature = "avx512f"),
            not(target_feature = "avx10.1")
        ))]
        return Some(ImbArch::IMB_ARCH_AVX2);
        
        // Check for SSE 4.2
        #[cfg(all(
            target_feature = "sse4.2",
            not(target_feature = "avx2"),
            not(target_feature = "avx512f"),
            not(target_feature = "avx10.1")
        ))]
        return Some(ImbArch::IMB_ARCH_SSE);
        
        // No specific features detected, use runtime detection
        None
    }
}