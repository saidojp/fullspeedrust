// Primitive Data Types in Rust
//int, float, boolean, char

//int бывает signed (+ и -) и unsigned (only +*)
//signed: i8, i16, i32, i64, i128 ... ＊положительные и так же отрицательные＊
//unsigned: u8, u16, u32, u64, u128 ＊только блять положительные＊

fn main() {
    let x: i32 = -43;
    let y: u32 = 53;

    println!("Signed int: {}, {}", x, y);
    println!("Unsigned int: {}, {}", y, x);
}
