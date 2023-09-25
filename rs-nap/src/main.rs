#![no_std]
#![no_main]
#![feature(naked_functions)]

mod support;
use support::*;
use core::slice::from_raw_parts as mkslice;

#[no_mangle]
pub unsafe fn nap(args: &[*const u8]) -> ! {
    let mut sleep_time:usize = 10;
    let mut good_input:bool = false;

    if args.len() > 1 {
        (sleep_time, good_input) = get_sleep_time(args[1]);
    }

    sleep(sleep_time, good_input);

    print_str(b"Done!\n");
    sys_exit(0)
}

pub unsafe fn sleep(mut sleep_time: usize, good_input: bool) {
    if good_input == false {
        sleep_time = 10;
        print_str(b"Bad input. ");
    }

    print_str(b"Sleeping for ");
    print_num(sleep_time);
    print_str(b" seconds...\n");

    sys_sleep(sleep_time);
}

unsafe fn get_sleep_time(arg: *const u8) -> (usize, bool) {
    let (seconds,_) = from_radix_10(mkslice(arg, strlen(arg)));
    (seconds, seconds > 0 && seconds < 1000000000)
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
