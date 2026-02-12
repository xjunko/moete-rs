/// trims out unused memory back to the OS (Linux only)
pub fn trim_memory() {
    #[cfg(target_os = "linux")]
    unsafe {
        libc::malloc_trim(0);
    }
}
