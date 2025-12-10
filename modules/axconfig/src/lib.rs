//! Platform-specific constants and parameters for [ArceOS].
//!
//! Currently supported platform configs can be found in the [configs] directory of
//! the [ArceOS] root.
//!
//! [ArceOS]: https://github.com/arceos-org/arceos
//! [configs]: https://github.com/arceos-org/arceos/tree/main/configs

#![no_std]

#[cfg(target_arch = "x86_64")]
#[path = "arch/x86_64.rs"]
pub mod arch;

#[cfg(target_arch = "riscv64")]
#[path = "arch/riscv64.rs"]
pub mod arch;

pub use crate::arch::*;