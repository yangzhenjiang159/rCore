//! 内存管理器具体实现
//!
//! 适用于RV64系统的SV39基于页的虚拟内存架构，以及所有与内存管理相关的内容，如帧分配器、页表、映射区域和内存集，都在此处实现。
//! 每个任务或进程都有一个内存集（memory_set）来控制其虚拟内存。

mod address;
mod frame_allocator;
mod heap_allocator;
mod memory_set;
mod page_table;

pub use address::{PhysAddr, PhysPageNum, VirtAddr, VirtPageNum};
use address::{StepByOne, VPNRange};
pub use frame_allocator::{frame_alloc, FrameTracker};
pub use memory_set::remap_test;
pub use memory_set::{MapPermission, MemorySet, KERNEL_SPACE};
pub use page_table::{translated_byte_buffer, PageTableEntry};
use page_table::{PTEFlags, PageTable};

/// 初始化堆分配器，帧分配器 和 内核空间
pub fn init() {
    heap_allocator::init_heap();
    frame_allocator::init_frame_allocator();
    KERNEL_SPACE.exclusive_access().activate();
}
