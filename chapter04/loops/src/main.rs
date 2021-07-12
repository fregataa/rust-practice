use std::io;

fn main() {
    let mut option = String::new();

    println!("Enter the input: ");
    io::stdin()
    .read_line(&mut option)
    .expect("Fail to read line!");

    let option: u32 = match option.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };

    if option == 0 {
        println!("The result is {}", loop_fn(10))
    } else if option == 1 {
        while_fn(10);
    } else {
        for_fn();
    }
}

fn loop_fn(count: u32) -> u32{
    let mut counter = 0;
    
    let result = loop {
        counter += 1;

        if counter == count {
            break counter * 2;
        }
    };

    result
}

fn while_fn(count: u32){
    let mut counter = 0;

    while counter < count {
        println!("Count up... {}", counter);
        counter += counter;
    }
}

fn for_fn(){
    let a = [10, 20, 30, 40, 50];

    for elem in a.iter() {
        println!("the value is {}", elem);
    }
}

