extern "C" {
    fn syscall3(syscall_number: usize, arg1: usize, arg2: usize, arg3: usize) -> usize;
    fn syscall1(syscall_number: usize, arg1: usize) -> usize;
}

const READ: usize = 0;
const WRITE: usize = 1;
const EXIT: usize = 60;

pub fn exit(code: usize) -> ! {
    unsafe { syscall1(EXIT, code); }
    loop {}
}

pub fn read(fd: usize, buff: &mut [u8]) -> usize {
    let ptr = buff.as_ptr();
    unsafe { syscall3(READ, fd, ptr as usize, buff.len()) }
}

pub fn write(fd: usize, buff: &[u8]) -> usize {
    let ptr = buff.as_ptr();
    unsafe { syscall3(WRITE, fd, ptr as usize, buff.len()) }
}
