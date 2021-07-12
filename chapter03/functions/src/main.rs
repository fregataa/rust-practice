fn main() {
    let x = 5;

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of x is {}", x);
    println!("The value of y is {}", y);
    println!("The value of x + y is {}", plus(x, y));
}

fn plus(x: i32, y: i32) -> i32 {
    x + y
}