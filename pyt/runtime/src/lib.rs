extern "C" {
    fn syscall3(syscall_number: usize, arg1: usize, arg2: usize, arg3: usize) -> usize;
    fn syscall1(syscall_number: usize, arg1: usize) -> usize;
}

fn exit(code: usize) -> ! {
    unsafe { syscall1(60, code); }
    loop {}
}


#[no_mangle]
pub extern "C" fn main() {
    let hello = b"Hello, World!\n";
    let ptr = hello as *const u8 as usize;
    unsafe {
        syscall3(1, 1, ptr, hello.len());
    }
    exit(0)
}