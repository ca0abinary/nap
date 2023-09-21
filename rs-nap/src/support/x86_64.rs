use core::arch::asm;

pub unsafe fn sys_exit(exit_code:usize) -> ! {
    asm!("syscall",
            in("rax") 60,
            in("rdi") exit_code,
            options(nostack, noreturn)
    )
}

pub unsafe fn sys_write(buffer: *const u8, count: usize) {
    asm!("syscall",
            inout("rax") 1 => _,
            in("rdi") 1,
            in("rsi") buffer,
            in("rdx") count,
            lateout("rcx") _,
            lateout("r11") _,
            options(nostack)
    )
}

#[repr(C)]
struct timespec {
    pub tv_sec:  isize,
    pub tv_nsec: isize,
}

pub unsafe fn sys_sleep(seconds: usize) {
    let sleep_time = timespec {
        tv_sec:  seconds as isize,
        tv_nsec: 0
    };

    asm!("syscall",
            in("rax") 35,
            in("rdi") &sleep_time,
            in("rsi") 0,
            options(nostack))
}
