//! 围绕`__switch`的Rust封装器。
//!
//! 切换到不同任务的上下文在此处进行。
//! 实际实现不能用Rust编写，并且（本质上）必须用汇编语言编写（你知道为什么吗？），
//! 所以这个模块实际上只是`switch.S`的一个包装器。

use super::TaskContext;
use core::arch::global_asm;

global_asm!(include_str!("switch.S"));

unsafe extern "C" {
    /// 切换到`next_task_cx_ptr`的上下文，并将当前上下文保存到`current_task_cx_ptr`中。
    pub unsafe fn __switch(
        current_task_cx_ptr: *mut TaskContext,
        next_task_cx_ptr: *const TaskContext,
    );
}