//! TaskContext的实现
use crate::trap::trap_return;

/// 包含一些寄存器的任务上下文结构
#[repr(C)]
pub struct TaskContext {
    /// __switch 汇编函数的返回地址（例如 __restore）
    ra: usize,
    /// 应用程序的内核栈指针
    sp: usize,
    /// 被调用者保存的寄存器：s 0..11
    s: [usize; 12],
}

impl TaskContext {
    /// 初始化 task context
    pub fn zero_init() -> Self {
        Self {
            ra: 0,
            sp: 0,
            s: [0; 12],
        }
    }

    /// set Task Context{__restore ASM funciton: trap_return, sp: kstack_ptr, s: s_0..12}
    /// 设置任务上下文{__restore汇编函数：trap_return，栈指针：内核栈指针，s寄存器：s_0..12}
    pub fn goto_trap_return(kstack_ptr: usize) -> Self {
        Self {
            ra: trap_return as usize,
            sp: kstack_ptr,
            s: [0; 12],
        }
    }
}