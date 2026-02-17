fn main() {
    let amount: i8 = -122;
    let another_amount: u8 = 255;
    println!("The amount is: {}, {}", amount, another_amount);
    main2();
}

fn main2() {
    let x: f64 = 0.2 + 0.3;
    println!("Sum is: {}", x);
    sum();
}

fn sum() {
    let j = 1.0;
    let o: f64 = 0.6;
    let jo = j + o;
    println!("sum ami: {}", jo);
}
