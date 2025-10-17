//! TaskContext的实现

/// Task Context
#[derive(Copy, Clone)]
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

    /// 设置任务上下文{__restore汇编函数、内核栈、s_0..12}
    pub fn goto_restore(kstack_ptr: usize) -> Self {
        unsafe extern "C" {
            fn __restore();
        }
        Self {
            ra: __restore as usize,
            sp: kstack_ptr,
            s: [0; 12],
        }
    }
}