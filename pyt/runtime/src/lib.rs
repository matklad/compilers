use std::io;

#[no_mangle]
pub extern "C" fn input() -> i32 {
    let mut buff = String::new();
    io::stdin().read_line(&mut buff).unwrap();
    let number = buff.trim();
    number.parse().unwrap()
}

#[no_mangle]
pub extern "C" fn print(x: i32) {
    println!("{}", x);
}