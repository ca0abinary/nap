#[repr(C)]
pub struct timespec {
    pub tv_sec:  isize,
    pub tv_nsec: isize,
}
