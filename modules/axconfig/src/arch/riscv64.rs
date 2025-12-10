#[doc = " Architecture identifier."]
pub const ARCH: &str = "riscv64";
#[doc = " Platform package."]
pub const PACKAGE: &str = "axplat-riscv64-qemu-virt";
#[doc = " Platform identifier."]
pub const PLATFORM: &str = "riscv64-qemu-virt";
#[doc = " Stack size of each task."]
pub const TASK_STACK_SIZE: usize = 0x40000;
#[doc = " Number of timer ticks per second (Hz). A timer tick may contain several timer"]
#[doc = " interrupts."]
pub const TICKS_PER_SEC: usize = 100;
#[doc = ""]
#[doc = " Device specifications"]
#[doc = ""]
pub mod devices {
    #[doc = " MMIO regions with format (`base_paddr`, `size`)."]
    pub const MMIO_REGIONS: &[(usize, usize)] = &[
        (0x0010_1000, 0x1000),         // RTC
        (0x0c00_0000, 0x21_0000),      // PLIC
        (0x1000_0000, 0x1000),         // UART
        (0x1000_1000, 0x8000),         // VirtIO
        (0x3000_0000, 0x1000_0000),    // PCI config space
        (0x4000_0000, 0x4000_0000),    // PCI memory ranges (ranges 1: 32-bit MMIO space)
    ];
    #[doc = " End PCI bus number."]
    pub const PCI_BUS_END: usize = 0xff;
    #[doc = " Base physical address of the PCIe ECAM space."]
    pub const PCI_ECAM_BASE: usize = 0x3000_0000;
    #[doc = " PCI device memory ranges."]
    pub const PCI_RANGES: &[(usize, usize)] = &[
        (0x0300_0000, 0x1_0000),        // PIO space
        (0x4000_0000, 0x4000_0000),     // 32-bit MMIO space
        (0x4_0000_0000, 0x4_0000_0000), // 64-bit MMIO space
    ];
    #[doc = " Timer interrupt num (PPI, physical timer)."]
    pub const TIMER_IRQ: usize = 0x8000_0000_0000_0005;
    #[doc = " VirtIO MMIO regions with format (`base_paddr`, `size`)."]
    pub const VIRTIO_MMIO_RANGES: &[(usize, usize)] = &[
        (0x1000_1000, 0x1000),
        (0x1000_2000, 0x1000),
        (0x1000_3000, 0x1000),
        (0x1000_4000, 0x1000),
        (0x1000_5000, 0x1000),
        (0x1000_6000, 0x1000),
        (0x1000_7000, 0x1000),
        (0x1000_8000, 0x1000),
    ];
}
#[doc = ""]
#[doc = " Platform configs"]
#[doc = ""]
pub mod plat {
    #[doc = " Number of CPUs."]
    pub const CPU_NUM: usize = 16;
    #[doc = " Platform family (deprecated)."]
    pub const FAMILY: &str = "";
    #[doc = " Kernel address space base."]
    pub const KERNEL_ASPACE_BASE: usize = 0xffff_ffc0_0000_0000;
    #[doc = " Kernel address space size."]
    pub const KERNEL_ASPACE_SIZE: usize = 0x0000_003f_ffff_f000;
    #[doc = " No need."]
    pub const KERNEL_BASE_PADDR: usize = 0x8020_0000;
    #[doc = " Base virtual address of the kernel image."]
    pub const KERNEL_BASE_VADDR: usize = 0xffff_ffc0_8020_0000;
    #[doc = " Offset of bus address and phys address. some boards, the bus address is"]
    #[doc = " different from the physical address."]
    pub const PHYS_BUS_OFFSET: usize = 0;
    #[doc = " No need."]
    pub const PHYS_MEMORY_BASE: usize = 0x8000_0000;
    #[doc = " No need."]
    pub const PHYS_MEMORY_SIZE: usize = 0x800_0000;
    #[doc = " No need."]
    pub const PHYS_VIRT_OFFSET: usize = 0xffff_ffc0_0000_0000;
}
