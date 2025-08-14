use core::arch::{asm, naked_asm};

#[no_mangle]
#[cfg(target_os = "macos")]
unsafe extern "C" fn main(argc: isize, argv: *const *const u8) -> ! {
    let args = core::slice::from_raw_parts(argv, argc as usize);
    crate::nap(args);
}

#[no_mangle]
#[unsafe(naked)]
#[cfg(not(target_os = "macos"))]
unsafe extern "C" fn _start() {
    #[cfg(target_arch = "aarch64")]
    naked_asm!(
        "mov x0, sp",
        "bl get_args"
    );
    
    #[cfg(target_arch = "x86_64")]
    naked_asm!(
        "mov rdi, rsp",
        "call get_args"
    );
}

pub unsafe fn sys_exit(exit_code: usize) -> ! {
    #[cfg(target_os = "macos")]
    {
        #[cfg(target_arch = "aarch64")]
        asm!("svc #0x80",
             in("x16") 1,  // SYS_exit on macOS
             in("x0") exit_code,
             options(nostack, noreturn)
        );
        
        #[cfg(target_arch = "x86_64")]
        asm!("syscall",
             in("rax") 0x2000001,  // SYS_exit on macOS
             in("rdi") exit_code,
             options(nostack, noreturn)
        );
    }
    
    #[cfg(not(target_os = "macos"))]
    {
        #[cfg(target_arch = "aarch64")]
        asm!("svc 0",
             in("w8") 93,
             in("x0") exit_code,
             options(nostack, noreturn)
        );
        
        #[cfg(target_arch = "x86_64")]
        asm!("syscall",
             in("rax") 60,
             in("rdi") exit_code,
             options(nostack, noreturn)
        );
    }
}

pub unsafe fn sys_write(buffer: *const u8, count: usize) {
    #[cfg(target_os = "macos")]
    {
        #[cfg(target_arch = "aarch64")]
        asm!("svc #0x80",
             in("x16") 4,  // SYS_write on macOS
             in("x0") 1,   // stdout
             in("x1") buffer,
             in("x2") count,
             lateout("x0") _,
             options(nostack)
        );
        
        #[cfg(target_arch = "x86_64")]
        asm!("syscall",
             in("rax") 0x2000004,  // SYS_write on macOS
             in("rdi") 1,          // stdout
             in("rsi") buffer,
             in("rdx") count,
             lateout("rax") _,
             lateout("rcx") _,
             lateout("r11") _,
             options(nostack)
        );
    }
    
    #[cfg(not(target_os = "macos"))]
    {
        #[cfg(target_arch = "aarch64")]
        asm!("svc #0",
             inout("x0") 1 => _,
             inout("x1") buffer => _,
             inout("x2") count => _,
             inout("x8") 64 => _,
             options(nostack)
        );
        
        #[cfg(target_arch = "x86_64")]
        asm!("syscall",
             inout("rax") 1 => _,
             in("rdi") 1,
             in("rsi") buffer,
             in("rdx") count,
             lateout("rcx") _,
             lateout("r11") _,
             options(nostack)
        );
    }
}

pub unsafe fn sys_sleep(seconds: usize) {
    #[cfg(target_os = "macos")]
    {
        #[cfg(target_arch = "aarch64")]
        asm!("svc #0x80",
             in("x16") 27,  // SYS_nanosleep on macOS
             in("x0") seconds,
             in("x1") 0,
             options(nostack)
        );
        
        #[cfg(target_arch = "x86_64")]
        asm!("syscall",
             in("rax") 0x200001b,  // SYS_nanosleep on macOS
             in("rdi") seconds,
             in("rsi") 0,
             options(nostack)
        );
    }
    
    #[cfg(not(target_os = "macos"))]
    {
        use super::interop::timespec;
        let sleep_time = timespec {
            tv_sec: seconds as isize,
            tv_nsec: 0
        };
        
        #[cfg(target_arch = "aarch64")]
        asm!("svc 0",
             in("x0") &sleep_time,
             in("x1") 0,
             in("x8") 101,
             options(nostack, preserves_flags)
        );
        
        #[cfg(target_arch = "x86_64")]
        asm!("syscall",
             in("rax") 35,
             in("rdi") &sleep_time,
             in("rsi") 0,
             options(nostack)
        );
    }
}