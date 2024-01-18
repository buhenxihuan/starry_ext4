extern crate alloc;
use crate::test::get_args;
use alloc::sync::Arc;
use axhal::arch::write_page_table_root;
use axhal::KERNEL_PROCESS_ID;
use axprocess::link::{create_link, FilePath};
use axprocess::{wait_pid, yield_now_task, PID2PC};
use axruntime::KERNEL_PAGE_TABLE;
use axtask::{TaskId, EXITED_TASKS};

fn busybox_fs_init() {
    create_link(
        &(FilePath::new("/usr/bin/busybox").unwrap()),
        &(FilePath::new("busybox").unwrap()),
    );
    create_link(
        &(FilePath::new("/usr/bin/ls").unwrap()),
        &(FilePath::new("busybox").unwrap()),
    );
    create_link(
        &(FilePath::new("/usr/bin/sh").unwrap()),
        &(FilePath::new("/busybox").unwrap()),
    );
    create_link(
        &(FilePath::new("/usr/bin/help").unwrap()),
        &(FilePath::new("/busybox").unwrap()),
    );
    create_link(
        &(FilePath::new("/usr/bin/cat").unwrap()),
        &(FilePath::new("/busybox").unwrap()),
    );
    create_link(
        &(FilePath::new("/usr/bin/exit").unwrap()),
        &(FilePath::new("/busybox").unwrap()),
    );
    create_link(
        &(FilePath::new("/usr/bin/mkdir").unwrap()),
        &(FilePath::new("/busybox").unwrap()),
    );
    create_link(
        &(FilePath::new("/usr/bin/pwd").unwrap()),
        &(FilePath::new("/busybox").unwrap()),
    );
    create_link(
        &(FilePath::new("/usr/bin/ll").unwrap()),
        &(FilePath::new("/busybox").unwrap()),
    );
    create_link(
        &(FilePath::new("/usr/bin/uname").unwrap()),
        &(FilePath::new("/busybox").unwrap()),
    );

    create_link(
        &FilePath::new("/usr/bin/basename").unwrap(),
        &FilePath::new("/busybox").unwrap(),
    );

    create_link(
        &FilePath::new("/usr/bin/cal").unwrap(),
        &FilePath::new("/busybox").unwrap(),
    );

    create_link(
        &FilePath::new("/usr/bin/clear").unwrap(),
        &FilePath::new("/busybox").unwrap(),
    );

    create_link(
        &FilePath::new("/usr/bin/date").unwrap(),
        &FilePath::new("/busybox").unwrap(),
    );

    create_link(
        &FilePath::new("/usr/bin/df").unwrap(),
        &FilePath::new("/busybox").unwrap(),
    );

    create_link(
        &FilePath::new("/usr/bin/dirname").unwrap(),
        &FilePath::new("/busybox").unwrap(),
    );

    create_link(
        &FilePath::new("/usr/bin/dmesg").unwrap(),
        &FilePath::new("/busybox").unwrap(),
    );

    create_link(
        &FilePath::new("/usr/bin/du").unwrap(),
        &FilePath::new("/busybox").unwrap(),
    );

    create_link(
        &FilePath::new("/usr/bin/expr").unwrap(),
        &FilePath::new("/busybox").unwrap(),
    );

    create_link(
        &FilePath::new("/usr/bin/false").unwrap(),
        &FilePath::new("/busybox").unwrap(),
    );

    create_link(
        &FilePath::new("/usr/bin/true").unwrap(),
        &FilePath::new("/busybox").unwrap(),
    );

    create_link(
        &FilePath::new("/usr/bin/which").unwrap(),
        &FilePath::new("/busybox").unwrap(),
    );

    create_link(
        &FilePath::new("/usr/bin/uname").unwrap(),
        &FilePath::new("/busybox").unwrap(),
    );

    create_link(
        &FilePath::new("/usr/bin/uptime").unwrap(),
        &FilePath::new("/busybox").unwrap(),
    );

    create_link(
        &FilePath::new("/usr/bin/printf").unwrap(),
        &FilePath::new("/busybox").unwrap(),
    );

    create_link(
        &FilePath::new("/usr/bin/ps").unwrap(),
        &FilePath::new("/busybox").unwrap(),
    );

    create_link(
        &FilePath::new("/usr/bin/free").unwrap(),
        &FilePath::new("/busybox").unwrap(),
    );

    create_link(
        &FilePath::new("/usr/bin/hwclock").unwrap(),
        &FilePath::new("/busybox").unwrap(),
    );

    create_link(
        &FilePath::new("/usr/bin/kill").unwrap(),
        &FilePath::new("/busybox").unwrap(),
    );

    create_link(
        &FilePath::new("/usr/bin/sleep").unwrap(),
        &FilePath::new("/busybox").unwrap(),
    );

    create_link(
        &FilePath::new("/usr/bin/touch").unwrap(),
        &FilePath::new("/busybox").unwrap(),
    );

    create_link(
        &FilePath::new("/usr/bin/cut").unwrap(),
        &FilePath::new("/busybox").unwrap(),
    );

    create_link(
        &FilePath::new("/usr/bin/od").unwrap(),
        &FilePath::new("/busybox").unwrap(),
    );

    create_link(
        &FilePath::new("/usr/bin/head").unwrap(),
        &FilePath::new("/busybox").unwrap(),
    );

    create_link(
        &FilePath::new("/usr/bin/tail").unwrap(),
        &FilePath::new("/busybox").unwrap(),
    );

    create_link(
        &FilePath::new("/usr/bin/hexdump").unwrap(),
        &FilePath::new("/busybox").unwrap(),
    );

    create_link(
        &FilePath::new("/usr/bin/md5sum").unwrap(),
        &FilePath::new("/busybox").unwrap(),
    );

    create_link(
        &FilePath::new("/usr/bin/sort").unwrap(),
        &FilePath::new("/busybox").unwrap(),
    );

    create_link(
        &FilePath::new("/usr/bin/stat").unwrap(),
        &FilePath::new("/busybox").unwrap(),
    );

    create_link(
        &FilePath::new("/usr/bin/strings").unwrap(),
        &FilePath::new("/busybox").unwrap(),
    );

    create_link(
        &FilePath::new("/usr/bin/wc").unwrap(),
        &FilePath::new("/busybox").unwrap(),
    );

    create_link(
        &FilePath::new("/usr/bin/more").unwrap(),
        &FilePath::new("/busybox").unwrap(),
    );

    create_link(
        &FilePath::new("/usr/bin/mv").unwrap(),
        &FilePath::new("/busybox").unwrap(),
    );

    create_link(
        &FilePath::new("/usr/bin/rmdir").unwrap(),
        &FilePath::new("/busybox").unwrap(),
    );

    create_link(
        &FilePath::new("/usr/bin/grep").unwrap(),
        &FilePath::new("/busybox").unwrap(),
    );

    create_link(
        &FilePath::new("/usr/bin/cp").unwrap(),
        &FilePath::new("/busybox").unwrap(),
    );

    create_link(
        &FilePath::new("/usr/bin/rm").unwrap(),
        &FilePath::new("/busybox").unwrap(),
    );

    create_link(
        &FilePath::new("/usr/bin/find").unwrap(),
        &FilePath::new("/busybox").unwrap(),
    );
}

pub fn run_shell() {
    busybox_fs_init(); // Assuming "sdcard" is the common case for running the shell
    #[allow(unused)]
    let mut ans = None;
    let args = get_args("./busybox sh busybox_testcode.sh".as_bytes());

    let main_task = axprocess::Process::init(args).unwrap();
    let now_process_id = main_task.get_process_id() as isize;
    let mut exit_code = 0;
    unsafe {
        ans = loop {
            if wait_pid(now_process_id, &mut exit_code as *mut i32).is_ok() {
                break Some(exit_code);
            }
            yield_now_task();
        };
    }
    TaskId::clear();
    unsafe {
        write_page_table_root(KERNEL_PAGE_TABLE.root_paddr());
        use riscv;
        riscv::asm::sfence_vma_all();
    };
    EXITED_TASKS.lock().clear();
    if let Some(_) = ans {
        let kernel_process = Arc::clone(PID2PC.lock().get(&KERNEL_PROCESS_ID).unwrap());
        kernel_process
            .children
            .lock()
            .retain(|x| x.pid() == KERNEL_PROCESS_ID);
    }
    axlog::ax_println!("bye~");
}
