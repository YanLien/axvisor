内存分配问题

用0的方式不对

```
[  0.228317 0:2 axvisor::vmm::config:180] Creating VM[1] "arceos-qemu"
[  0.229266 0:2 axvm::vm:152] VM created: id=1
[  0.229597 0:2 axvisor::vmm::config:262] VM[1] 1 map same region: VmMemConfig {
    gpa: 0xc1200000,
    size: 0x8000000,
    flags: 0x7,
    map_type: MapIdentical,
}
[  0.235767 0:2 axalloc:232] expand heap memory: [0xffffffc08115d000, 0xffffffc08915d000)
[  0.247015 0:2 axalloc:232] expand heap memory: [0xffffffc08915d000, 0xffffffc09915d000)
[  0.302739 0:2 axaddrspace::address_space:84] xxxxxxxxx
[  0.303064 0:2 axaddrspace::address_space::backend::linear:22] map_linear: [GPA:0x89200000, GPA:0x91200000) -> [PA:0x89200000, PA:0x91200000) READ | WRITE | EXECUTE | USER
[  0.304865 0:2 axvisor::vmm::config:214] config_guest_address: main_memory.gpa=GPA:0x89200000, is_identical=false
[  0.305219 0:2 axvisor::vmm::config:220] Before adjustment: bsp_entry=GPA:0xc1400000, kernel_load_gpa=GPA:0xc1400000
[  0.305504 0:2 axvisor::vmm::config:241] After adjustment: bsp_entry=GPA:0xc1400000, kernel_load_gpa=GPA:0xc1400000
[  0.305788 0:2 axvisor::vmm::config:198] VM[1] created success, loading images...
[  0.306303 0:2 axvisor::vmm::images:66] Loading VM[1] images into memory region: gpa=GPA:0x89200000, hva=VA:0xffffffc089200000, size=128 MiB
[  0.308163 0:2 axvisor::vmm::images:94] Loading VM[1] images from memory
[  0.308418 0:2 axvisor::vmm::images:143] loading VM image from memory GPA:0xc1400000 61504
[  0.309038 0:2 axruntime::lang_items:5] panicked at /home/yanlien/Program/riscv64/axvisor/crates/axvm/src/vm.rs:405:14:
Failed to translate kernel image load address
[  0.309550 0:2 axplat_riscv64_qemu_virt::power:25] Shutting down...
```

问题

+ 系统给虚拟机分配了内存：`[GPA:0x89200000, GPA:0x91200000)`
+ 但内核要加载到：`kernel_load_gpa=0xc1400000`
+ `0xc1400000` 不在分配的内存范围内，所以程序崩溃

启动虚拟机的问题

``` bash
[  0.338411 0:2 axvm::vm:416] Booting VM[1]
[  0.339178 0:2 axvisor::vmm:62] VM[1] boot success
[  0.339583 0:2 axvisor::vmm:73] a VM exited, current running VM count: 1
[  0.340356 0:2 axvisor::vmm:73] a VM exited, current running VM count: 1
[  0.340919 0:2 axtask::run_queue:416] task block: Task(2, "main")
[  0.341638 0:3 axtask::task:455] task drop: Task(4, "")
[  0.342043 0:3 axtask::run_queue:416] task block: Task(3, "gc")
[  0.342408 0:5 axvisor::vmm::vcpus:431] VM[1] boot delay: 0s
[  0.342671 0:5 axvisor::vmm::vcpus:434] VM[1] VCpu[0] waiting for running
[  0.343307 0:5 axvisor::vmm::vcpus:437] VM[1] VCpu[0] running...
[  0.344235 0:5 riscv_vcpu::vcpu:193] Unknown trap cause: scause=0x14
[  0.344527 0:5 axvisor::vmm::vcpus:549] VM[1] run VCpu[0] get error AxErrorKind::InvalidData
[  0.344914 0:5 axvm::vm:453] Shutting down VM[1]
[  0.345214 0:5 axvisor::vmm::vcpus:568] VM[1] VCpu[0] stopping because of VM stopping
[  0.345514 0:5 axvisor::vmm::vcpus:574] VM[1] VCpu[0] last VCpu exiting, decreasing running VM count
[  0.345782 0:5 axvisor::vmm::vcpus:578] VM[1] state changed to Stopped
[  0.346035 0:5 axtask::run_queue:266] task unblock: Task(2, "main") on run_queue 0
[  0.346645 0:2 axvisor::vmm:73] a VM exited, current running VM count: 0
[  0.346901 0:2 axvisor:32] [OK] Default guest initialized
[  0.347057 0:2 axtask::run_queue:363] task exit: Task(2, "main"), exit_code=0
[  0.347336 0:2 axplat_riscv64_qemu_virt::power:25] Shutting down...
```