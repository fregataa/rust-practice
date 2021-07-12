fn main() {
    let num = 10;

    if num % 3 == 0 {
        println!("number is divisible by 4");
    } else if num % 4 == 0 {
        println!("number is divisible by 3");
    } else if num % 5 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is NOT divisible by 4, 3 or 2");
    }

    let condition = true;
    let num = if condition {
        5
    } else {
        6
    };

    println!("The value of num is {}", num);
}
