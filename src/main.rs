use std::io;

struct User {
    name: String,
    age: isize,
    mobile_number: isize,
    sign_in_count: isize,
}

static mut COUNTER: isize = 0;

fn main() {
    let users = number_of_users();   // get vector of users

    println!("Total users entered: {}", users.len());

    // print all user data
    for u in &users {
        println!(
            "User: {}, Age: {}, Mobile: {}, Signins: {}",
            u.name, u.age, u.mobile_number, u.sign_in_count
        );
    }
}

fn struct_example() -> User {
    let user = User {
        name: name(),
        age: age(),
        mobile_number: mobile_number(),
        sign_in_count: signin(),
    };
    user
}

fn signin() -> isize {
    unsafe {
        COUNTER += 1;
        COUNTER
    }
}

fn name() -> String {
    println!("Enter your name: ");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
}

fn mobile_number() -> isize {
    println!("Enter your mobile number: ");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    match input.trim().parse::<isize>() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid mobile number, using 0");
            0
        }
    }
}

fn age() -> isize {
    println!("Enter your age: ");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    match input.trim().parse::<isize>() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid age, using 0");
            0
        }
    }
}

fn number_of_users() -> Vec<User> {
    println!("Enter number of users: ");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let count = match input.trim().parse::<isize>() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid number, using 0");
            0
        }
    };

    let mut users: Vec<User> = Vec::new();

    for _ in 0..count {
        let u = struct_example();
        users.push(u);
    }

    users
}
