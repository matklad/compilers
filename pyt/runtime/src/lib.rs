#![no_std]
#![feature(lang_items)]

mod ring_buffer;
mod syscall;

use ring_buffer::RingBuff;

#[no_mangle]
pub extern "C" fn input() -> u64 {
    static mut BUFFER: RingBuff = ring_buffer::EMPTY;

    fn parse_digit(b: u8) -> Option<u64> {
        if b'0' <= b && b <= b'9' {
            Some((b - b'0') as u64)
        } else {
            None
        }
    }

    let mut result;
    unsafe {
        'skip_leading_ws: loop {
            while let Some(b) = BUFFER.next() {
                if let Some(d) = parse_digit(b) {
                    result = d;
                    break 'skip_leading_ws;
                }
            }
            let n = syscall::read(0, BUFFER.buff());
            BUFFER.advance(n)
        }

        'read_digits: loop {
            while let Some(b) = BUFFER.next() {
                if let Some(d) = parse_digit(b) {
                    result = result * 10 + d;
                } else {
                    break 'read_digits;
                }
            }

            let n = syscall::read(0, BUFFER.buff());
            BUFFER.advance(n)
        }
    }
    result
}


#[no_mangle]
pub extern "C" fn print(n: u64) {
    if n == 0 {
        syscall::write(1, &[b'0']);
        return;
    }
    let mut buff = [0u8; 64];
    let mut idx = buff.len();
    let mut n = n;
    while n > 0 {
        idx -= 1;
        buff[idx] = b'0' + (n % 10) as u8;
        n /= 10;
    }
    syscall::write(1, &buff[idx..]);
}

#[no_mangle]
pub extern "C" fn exit() -> ! {
    syscall::exit(0)
}


//#[lang = "panic_fmt"]
//#[no_mangle]
//pub extern fn rust_begin_panic(_msg: core::fmt::Arguments,
//                               _file: &'static str,
//                               _line: u32) -> ! {
//    syscall::write(2, b"Panic!\n");
//    syscall::exit(92)
//}