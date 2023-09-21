use core::arch::asm;
use core::slice::from_raw_parts as mkslice;

mod interop;
use interop::timespec;

#[no_mangle]
#[naked]
unsafe extern "C" fn _start() {
    // Move the stack pointer before it gets clobbered
    asm!("mov fp, sp",
         "mov x0, fp",
         "bl get_args",
         options(noreturn))
}

pub unsafe fn sys_exit(exit_code:usize) -> ! {
    asm!("svc 0",
         in("x8") 93,
         in("x0") exit_code,
         options(nostack, noreturn)
    )
}

pub unsafe fn sys_write(buffer: *const u8, count: usize) {
    asm!("svc 0", "ret",
         in("x8") 64,
         inlateout("x0") 1 => _,
         in("x1") buffer,
         in("x2") count,
         options(nostack, preserves_flags)
    )
}

pub unsafe fn sys_sleep(seconds: usize) {
    let sleep_time = timespec {
        tv_sec:  seconds as isize,
        tv_nsec: 0
    };

    asm!("svc 0", "ret",
         inlateout("x0") &sleep_time => _,
         in("x1") 0,
         in("x8") 101,
         options(nostack, preserves_flags)
    )
}
