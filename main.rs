use std::io;
use std::thread;
use std::time::Duration;

fn main() {

    // GREETING HANDLER
    println!("Welcome to the rust calculator! (Made by Hasan M. Hasan </3)");
    thread::sleep(Duration::from_secs(2));
    
    // INPUT HANDLER
    println!("Enter an operation (+, -, *, /):");
    thread::sleep(Duration::from_secs(2));
    let mut operation = String::new();
    io::stdin()
        .read_line(&mut operation)
        .expect("Failed to read line");
    let operation = operation.trim();


    // CALCULATION PROCEDURER & ERROR HANDLER (this is just bloatware at this point lol)
    if operation == "+" {
        println!("You have chosen addition!");
        thread::sleep(Duration::from_secs(1));
        println!("The addition module will load!");
        thread::sleep(Duration::from_secs(1));
        println!(".");
        thread::sleep(Duration::from_secs(1));
        println!("..");
        thread::sleep(Duration::from_secs(1));
        println!("...");
        thread::sleep(Duration::from_secs(1));
        addition();
    } else if operation == "-" {
        println!("You have chosen subtraction!");
        thread::sleep(Duration::from_secs(1));
        println!("The subtraction module will load!");
        thread::sleep(Duration::from_secs(1));
        println!(".");
        thread::sleep(Duration::from_secs(1));
        println!("..");
        thread::sleep(Duration::from_secs(1));
        println!("...");
        thread::sleep(Duration::from_secs(1));
        subtraction();
    } else if operation == "*" {
        println!("You have chosen multipication!");
        thread::sleep(Duration::from_secs(1));
        println!("The multipication module will load!");
        thread::sleep(Duration::from_secs(1));
        println!(".");
        thread::sleep(Duration::from_secs(1));
        println!("..");
        thread::sleep(Duration::from_secs(1));
        println!("...");
        thread::sleep(Duration::from_secs(1));
        multipication();
    } else if operation == "/" {
        println!("You have chosen division!");
        thread::sleep(Duration::from_secs(1));
        println!("The division module will load!");
        thread::sleep(Duration::from_secs(1));
        println!(".");
        thread::sleep(Duration::from_secs(1));
        println!("..");
        thread::sleep(Duration::from_secs(1));
        println!("...");
        thread::sleep(Duration::from_secs(1));
        division();
    } else {
        println!("Unknown input, please input a valid operation.");
        thread::sleep(Duration::from_secs(1));
        println!("The program will start again!");
        thread::sleep(Duration::from_secs(1));
        main();
    }
}

// OPERATOR FUNCTIONS
fn addition(){
    println!("Please input the first number.");

    // INPUT HANDLER FOR INTEGER 1
    let mut ad_input_1 = String::new();
    io::stdin().read_line(&mut ad_input_1).expect("Failed to read input.");

    // STRING TO INTEGER CONVERTER
    let ad_input_1: f64 = ad_input_1.trim().parse().expect("Failed to read number.");
    thread::sleep(Duration::from_secs(1));
    println!("Great! Now input the second number.");

    // INPUT HANDLER FOR INTEGER 2
    let mut ad_input_2 = String::new();
    io::stdin().read_line(&mut ad_input_2).expect("Failed to read input.");

    // STRING TO INTEGER CONVERTER
    let ad_input_2: f64 = ad_input_2.trim().parse().expect("Failed to read number.");
    thread::sleep(Duration::from_secs(1));
    println!(".");
        thread::sleep(Duration::from_secs(1));
        println!("..");
        thread::sleep(Duration::from_secs(1));
        println!("...");
        thread::sleep(Duration::from_secs(1));

    // ADDITION PROCESSOR
    let ad_total = ad_input_1 + ad_input_2;
    println!("The addition result is {}!", ad_total);
    thread::sleep(Duration::from_secs(1));
    rerun();
}

fn subtraction(){
    println!("Please input the first number.");

    // INPUT HANDLER FOR INTEGER 1
    let mut sub_input_1 = String::new();
    io::stdin().read_line(&mut sub_input_1).expect("Failed to read input.");

    // STRING TO INTEGER CONVERTER
    let sub_input_1: f64 = sub_input_1.trim().parse().expect("Failed to read number.");
    thread::sleep(Duration::from_secs(1));
    println!("Great! Now input the second number.");

    // INPUT HANDLER FOR INTEGER 2
    let mut sub_input_2 = String::new();
    io::stdin().read_line(&mut sub_input_2).expect("Failed to read input.");

    // STRING TO INTEGER CONVERTER
    let sub_input_2: f64 = sub_input_2.trim().parse().expect("Failed to read number.");
    thread::sleep(Duration::from_secs(1));
    println!(".");
        thread::sleep(Duration::from_secs(1));
        println!("..");
        thread::sleep(Duration::from_secs(1));
        println!("...");
        thread::sleep(Duration::from_secs(1));

    // SUBTRACTION PROCESSOR
    let sub_total = sub_input_1 - sub_input_2;
    println!("The subtraction result is {}!", sub_total);
    thread::sleep(Duration::from_secs(1));
    rerun();
}

fn multipication(){
    println!("Please input the first number.");

    // INPUT HANDLER FOR INTEGER 1
    let mut mul_input_1 = String::new();
    io::stdin().read_line(&mut mul_input_1).expect("Failed to read input.");

    // STRING TO INTEGER CONVERTER
    let mul_input_1: f64 = mul_input_1.trim().parse().expect("Failed to read number.");
    thread::sleep(Duration::from_secs(1));
    println!("Great! Now input the second number.");

    // INPUT HANDLER FOR INTEGER 2
    let mut mul_input_2 = String::new();
    io::stdin().read_line(&mut mul_input_2).expect("Failed to read input.");

    // STRING TO INTEGER CONVERTER
    let mul_input_2: f64 = mul_input_2.trim().parse().expect("Failed to read number.");
    thread::sleep(Duration::from_secs(1));
    println!(".");
        thread::sleep(Duration::from_secs(1));
        println!("..");
        thread::sleep(Duration::from_secs(1));
        println!("...");
        thread::sleep(Duration::from_secs(1));

    // MULTIPICATION PROCESSOR
    let mul_total = mul_input_1 * mul_input_2;
    println!("The multipication result is {}!", mul_total);
    thread::sleep(Duration::from_secs(1));
    rerun();
}

fn division(){
    println!("Please input the first number.");

    // INPUT HANDLER FOR INTEGER 1
    let mut div_input_1 = String::new();
    io::stdin().read_line(&mut div_input_1).expect("Failed to read input.");

    // STRING TO INTEGER CONVERTER
    let div_input_1: f64 = div_input_1.trim().parse().expect("Failed to read number.");
    thread::sleep(Duration::from_secs(1));
    println!("Great! Now input the second number.");

    // INPUT HANDLER FOR INTEGER 2
    let mut div_input_2 = String::new();
    io::stdin().read_line(&mut div_input_2).expect("Failed to read input.");

    // STRING TO INTEGER CONVERTER
    let div_input_2: f64 = div_input_2.trim().parse().expect("Failed to read number.");
    thread::sleep(Duration::from_secs(1));
    println!(".");
        thread::sleep(Duration::from_secs(1));
        println!("..");
        thread::sleep(Duration::from_secs(1));
        println!("...");
        thread::sleep(Duration::from_secs(1));

    // DIVISION PROCESSOR
    let div_total = div_input_1 / div_input_2;
    println!("The multipication result is {}!", div_total);
    thread::sleep(Duration::from_secs(1));
    rerun();
}

fn rerun(){
    println!("Would you like to run this program again? (y/n)");

    // INPUT HANDLER
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read line.");

    let choice = choice.trim().to_lowercase();

    if choice == "y" {
        main();
        thread::sleep(Duration::from_secs(1));
    } else if choice == "n" {
        println!("Exitting...");
        thread::sleep(Duration::from_secs(1));
        std::process::exit(0);
    } else {
        println!("Please either enter (y) or (n)...");
        thread::sleep(Duration::from_secs(1));
        rerun();
    }
}
