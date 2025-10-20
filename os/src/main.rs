//! 主模块和入口点
//!
//! 内核的各种功能被实现为子模块。其中最重要的有：
//!
//! - [`trap`]：处理从用户空间切换到内核的所有情况
//! - [`task`]：任务管理
//! - [`syscall`]：系统调用处理与实现
//!
//! 操作系统也在这个模块中启动。
//! 内核代码从`entry.asm`开始执行，之后会调用[`rust_main()`]来初始化各种功能组件。
//! （详情请参见其源代码。）
//!
//! 然后我们调用 [`task::run_first_task()`]，并首次进入用户空间。

#![deny(missing_docs)]
#![deny(warnings)]
#![no_std]
#![no_main]

use core::arch::global_asm;
use log::*;
#[macro_use]
mod console;
mod config;
mod lang_items;
mod loader;
mod logging;
mod sbi;
mod sync;
pub mod syscall;
pub mod task;
mod timer;
pub mod trap;

global_asm!(include_str!("entry.asm"));
global_asm!(include_str!("link_app.S"));

/// clear BSS segment
fn clear_bss() {
    unsafe extern "C" {
        safe fn sbss();
        safe fn ebss();
    }
    unsafe {
        core::slice::from_raw_parts_mut(sbss as usize as *mut u8, ebss as usize - sbss as usize)
            .fill(0);
    }
}

/// the rust entry-point of os
#[unsafe(no_mangle)]
pub fn rust_main() -> ! {
    clear_bss();
    logging::init();
    info!("[kernel] Hello, world!");
    trap::init();
    loader::load_apps();
    trap::enable_timer_interrupt();
    timer::set_next_trigger();
    task::run_first_task();
    panic!("Unreachable in rust_main!");
}
