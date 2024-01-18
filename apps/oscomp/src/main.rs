#![no_std]
#![no_main]
// extern crate axstarry;

// use syscall_entry::run_testcases;
use syscall_entry::run_shell;

#[no_mangle]
fn main() {
    // run_testcases("sdcard");
    run_shell();
}
