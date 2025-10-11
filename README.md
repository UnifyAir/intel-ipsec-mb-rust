# Intel IPSec Multi-Buffer Crypto Library - Rust Bindings

[![Crates.io](https://img.shields.io/crates/v/intel-ipsec-mb.svg)](https://crates.io/crates/intel-ipsec-mb)
[![Documentation](https://docs.rs/intel-ipsec-mb/badge.svg)](https://docs.rs/intel-ipsec-mb)
[![License](https://img.shields.io/badge/license-BSD--3--Clause-blue.svg)](LICENSE)

High-performance Rust bindings for Intel's Multi-Buffer Crypto for IPsec Library, providing optimized cryptographic operations for packet processing applications.

## Overview

This project provides safe Rust bindings to Intel's optimized cryptographic library, which is designed for high-performance packet processing applications such as:

- **IPsec** - Internet Protocol Security
- **TLS** - Transport Layer Security  
- **Wireless (RAN)** - Radio Access Network
- **Cable** - Cable modem applications
- **MPEG DRM** - Digital Rights Management

## Key Features

- 🚀 **High Performance**: Leverages Intel's latest instruction extensions (AVX2, AVX-512, etc.)
- 🔒 **Safe Rust API**: Memory-safe wrappers around the C library
- ⚡ **Multi-Buffer Processing**: Advanced cryptographic pipelining
- 🔗 **Operation Chaining**: Combine encryption and authentication operations
- 🎯 **Job Management**: Built-in scheduling and dispatching functions
- 🏗️ **Cross-Platform**: Supports x86, x86_64, and aarch64 architectures

## Architecture

The project consists of two main crates:

### `intel-ipsec-mb-sys`
Low-level unsafe FFI bindings to the Intel IPSec MB C library. This crate:
- Generates Rust bindings using `bindgen`
- Builds the Intel IPSec MB library from source using CMake
- Provides raw access to all C library functions

### `intel-ipsec-mb` 
High-level safe Rust API that wraps the sys crate. This crate:
- Provides memory-safe abstractions
- Implements proper error handling
- Offers convenient APIs for common operations

## Supported Operations

### Hash Functions
- **SHA-1**: Secure Hash Algorithm 1
- **SHA-2**: SHA-224, SHA-256, SHA-384, SHA-512
- **MD5**: Message Digest 5

### Cipher Operations
- **AES**: Advanced Encryption Standard
- **3DES**: Triple Data Encryption Standard
- **ChaCha20**: Stream cipher

### Authentication
- **HMAC**: Hash-based Message Authentication Code
- **Poly1305**: Authenticator

## Quick Start

### Prerequisites

- **Rust**: 1.70+ (2024 edition)
- **CMake**: 3.18+
- **NASM**: 2.14+ (for assembly compilation)
- **Git**: For submodule initialization

### Installation

1. **Clone the repository**:
   ```bash
   git clone <repository-url>
   cd intel-ipsec-mb
   ```

2. **Initialize submodules**:
   ```bash
   git submodule update --init --recursive
   ```

3. **Build the project**:
   ```bash
   cargo build --release
   ```

### Basic Usage

```rust
use intel_ipsec_mb::mgr::MbMgr;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a new manager
    let mgr = MbMgr::new()?;
    
    // Prepare input data
    let input = b"Hello, World!";
    let mut output = vec![0u8; 20]; // SHA-1 output size
    
    // Compute SHA-1 hash
    mgr.sha1(input, &mut output)?;
    
    println!("SHA-1 hash: {:02x?}", output);
    Ok(())
}
```

### Advanced Usage with Job Management

```rust
use intel_ipsec_mb::mgr::MbMgr;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mgr = MbMgr::new()?;
    
    unsafe {
        // Get a job from the manager
        let mut job = mgr.get_next_job()?;
        let mut output = vec![0u8; 20];
        
        // Fill the job with data
        mgr.fill_job_sha1(&mut job, b"Hello, World!", &mut output)?;
        
        // Submit the job for processing
        mgr.submit_job()?;
        
        println!("Hash: {:02x?}", output);
    }
    
    Ok(())
}
```

## Configuration

The library supports various configuration options:

```rust
use intel_ipsec_mb::config::MbMgrConfig;

let config = MbMgrConfig::default()
    .with_architecture_detection(true)
    .with_forced_architecture(None);

let mgr = MbMgr::new_with_config(config)?;
```

## Build Configuration

### Features

- **`allow-forced`**: Enables runtime architecture selection (increases binary size)

### Environment Variables

- `PROFILE`: Controls build optimization (debug/release)
- `CARGO_CFG_TARGET_ARCH`: Target architecture (x86, x86_64, aarch64)

### CMake Options

The build system automatically configures CMake with:
- `BUILD_SHARED_LIBS=OFF`: Static library build
- `BUILD_LIBRARY_ONLY=ON`: Library-only build
- `SAFE_OPTIONS`: Debug/release specific safety options

## Supported Platforms

| Architecture | OS | Status |
|-------------|----|---------| 
| x86_64 | Linux | ✅ Supported |
| x86_64 | Windows | 🚧 In Progress |
| x86_64 | macOS | 🚧 In Progress |
| x86 | Linux | ✅ Supported |

## Performance

The Intel IPSec MB library provides significant performance improvements over standard cryptographic implementations:

- **Multi-buffer processing**: Process multiple operations in parallel
- **Instruction-level optimization**: Uses latest CPU instruction sets
- **Pipelining**: Advanced job scheduling and dispatching
- **Memory efficiency**: Optimized memory access patterns

## Error Handling

The library provides comprehensive error handling:

```rust
use intel_ipsec_mb::error::MbError;

match mgr.sha1(input, &mut output) {
    Ok(_) => println!("Operation successful"),
    Err(MbError::InvalidInput) => println!("Invalid input data"),
    Err(MbError::InsufficientBuffer) => println!("Output buffer too small"),
    Err(e) => println!("Other error: {}", e),
}
```

## Development Status

⚠️ **Note**: This project is currently in active development. See [TODO.md](TODO.md) for current development priorities and known issues.

### Current Limitations

- Some function pointers are wrapped as `Option<T>` and need proper handling
- Architecture-specific optimizations are not fully implemented
- Windows and macOS support is incomplete
- Error handling needs improvement in some areas

## Contributing

Contributions are welcome! Please see the [TODO.md](TODO.md) file for areas that need work.

### Development Setup

1. Install development dependencies:
   ```bash
   cargo install cargo-watch cargo-expand
   ```

2. Run tests:
   ```bash
   cargo test
   ```

3. Run examples:
   ```bash
   cargo run --example basic_usage
   ```

## License

This project is licensed under the BSD-3-Clause License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Intel Corporation for the original IPSec Multi-Buffer Crypto Library
- The Rust community for excellent tooling and ecosystem
- Contributors to this project

## Related Projects

- [Intel IPSec MB](https://github.com/intel/intel-ipsec-mb) - Original C library
- [DPDK](https://www.dpdk.org/) - Data Plane Development Kit
- [Intel QAT Engine](https://github.com/intel/QAT_Engine) - QuickAssist Technology
- [FD.io](https://fd.io/) - Fast Data Project

## Support

For issues and questions:
- Open an issue on GitHub
- Check the [TODO.md](TODO.md) for known issues
- Review the original Intel IPSec MB documentation

---

**Note**: This is a work in progress. The API may change as development continues.
