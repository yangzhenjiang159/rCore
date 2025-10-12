//! 系统调用的实现
//!
//! 所有系统调用的唯一入口点 [`syscall()`] 会在用户态希望使用 `ecall`
//! 指令执行系统调用时被调用。在这种情况下，处理器会触发一个“来自用户态的环境调用”异常，
//! 该异常会作为 [`crate::trap::trap_handler`] 中的一种情况进行处理。
//!
//! 为清晰起见，每个单独的系统调用都被实现为其自身的函数，命名方式为
//! `sys_` 后接系统调用的名称。你可以在子模块中找到诸如此类的函数，
//! 并且你也应该以这种方式实现系统调用。

const SYSCALL_WRITE: usize = 64;
const SYSCALL_EXIT: usize = 93;

mod fs;
mod process;

use fs::*;
use process::*;

/// 使用`syscall_id`和其他参数处理系统调用异常
pub fn syscall(syscall_id: usize, args: [usize; 3]) -> isize {
    match syscall_id {
        SYSCALL_WRITE => sys_write(args[0], args[1] as *const u8, args[2]),
        SYSCALL_EXIT => sys_exit(args[0] as i32),
        _ => panic!("Unsupported syscall_id: {}", syscall_id),
    }
}