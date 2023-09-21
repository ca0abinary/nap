#![no_std]
#![no_main]
#![feature(naked_functions)]

mod support;
use support::*;
use core::slice::from_raw_parts as mkslice;

#[no_mangle]
pub unsafe fn nap(args: &[*const u8]) -> ! {
    let mut sleep_time:usize = 10;

    if args.len() < 2 {
        sleep(sleep_time, false);
    } else {
        let (seconds,_) = from_radix_10(mkslice(args[1], strlen(args[1])));
        let mut good_input = seconds > 0 && seconds < 1000000000;

        if good_input {
            sleep_time = seconds;
        } else {
            good_input = false;
        }

        sleep(sleep_time, good_input);
    }

    print_str(b"Done!\n");
    sys_exit(0);
}

pub unsafe fn sleep(seconds: usize, good_input: bool) {
    if good_input == false {
        print_str(b"Bad input. ");
    }
    print_str(b"Sleeping for ");
    print_num(seconds);
    print_str(b" seconds...\n");
    sys_sleep(seconds)
}

#[no_mangle]
unsafe fn get_args(stack_top: *const u8) {
    let argc = *(stack_top as *const usize);
    let argv = stack_top.add(8) as *const *const u8;
    let args = mkslice(argv, argc as usize);
    nap(args)
}

#[panic_handler]
unsafe fn my_panic(_info: &core::panic::PanicInfo) -> ! {
    sys_exit(255)
}
