fn main() {
    println!("Hello, world!");
    let y = 5;
    let x = 15;
    another_function(y);

    println!("The value of y is: {}", y);
    println!("The value of x is: {}", x);

    let a = y > x;
    println!("The value of a is: {}", a);

    let z = {
        let f = 3;
        println!("oi");
        f + 1 + x + y + five()
    };
    println!("The value of z is: {}", z);

}

fn another_function(mut x: i32) {
    x += 5;
    println!("Another function.");
    println!("The value of x is: {x}");
}

fn five() -> i32 {
    5
}
