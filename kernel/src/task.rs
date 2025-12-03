use alloc::sync::{Arc, Weak};
use std::os::arceos::modules::axtask::{TaskExt, TaskInner};

/// Task extended data for the hypervisor.
pub struct VCpuTask {}

impl VCpuTask {}

#[extern_trait::extern_trait]
unsafe impl TaskExt for VCpuTask {}
