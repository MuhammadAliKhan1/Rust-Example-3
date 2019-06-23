extern crate rand;
use rand::Rng;
use std::io;
use std::io::{stdin, stdout, Read, Write};
use std::str::FromStr;
fn main() {
    loop {
        print!("\n1)Generate Random Numbers\n2)Generate Random Numbers with a Range\n3)Generate a Random String\n4)Sort a Vector of Integers.\n5)Sort vector of floats.\n6)Generate a float vector and count elements\n7)Generate a integer vector(Conditional)\n8)Number Guessing Game\n9)Dice\n10)Hi-Lo\n11)Adding values in a loop\n12)Calculator\nAny other number to exit.\nInput: ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let input = input.trim().parse::<u32>().expect("Input a number");
        clrscr();

        match input {
            1 => gen_rand_types(),
            2 => gen_rand_10(),
            3 => gen_rand_ascii(),
            4 => sort_vec_int(),
            5 => sort_vec_float(),
            6 => check_float_100(),
            7 => sort_90(),
            8 => guessing_game(),
            9 => dice(),
            10 => hi_lo(),
            11 => add_input(),
            12 => calculator(),
            _ => break,
        }
    }
}
fn clrscr() {
    for _ in 0..50 {
        println!();
    }
}
fn pause() {
    let mut stdout = stdout();
    stdout.write(b"Press Enter to continue...").unwrap();
    stdout.flush().unwrap();
    stdin().read(&mut [0]).unwrap();
}
fn gen_rand_types() {
    let num_1: u8 = rand::thread_rng().gen();
    let num_2: u16 = rand::thread_rng().gen();
    let num_3: u32 = rand::thread_rng().gen();
    let num_4: i32 = rand::thread_rng().gen();
    let num_5: f32 = rand::thread_rng().gen();
    println!(
        "\nRandom u8: {}\nRandom u16: {}\nRandom u32: {}\nRandom i32: {}\nRandom float: {}\n",
        num_1, num_2, num_3, num_4, num_5
    );
    pause();
}
fn gen_rand_10() {
    let num: u8 = rand::thread_rng().gen_range(0, 10);
    let num_1 = rand::thread_rng().gen_range(0.0, 10.0);
    println!("\nInteger: {}\nFloat: {}\n", num, num_1);
    pause();
}
fn gen_rand_ascii() {
    for _ in 0..30 {
        print!("{}", rand::thread_rng().gen_range(b'a', b'z') as char);
        print!("{}", rand::thread_rng().gen_range(0, 9));
        print!("{}", rand::thread_rng().gen_range(b'A', b'Z') as char);
    }
    println!();
    pause();
}
fn sort_vec_int() {
    print!("> ");
    io::stdout().flush().unwrap();
    let mut vec = String::new();
    io::stdin()
        .read_line(&mut vec)
        .expect("Failed to read line");
    vec.pop();
    vec.pop();
    let vec = &vec[1..];
    let vec: Vec<_> = vec.split(",").collect();
    let mut num_vec = Vec::new();
    for x in vec {
        let n: i32 = FromStr::from_str(x).unwrap();
        num_vec.push(n);
    }
    let size = num_vec.len();
    for _ in 0..size {
        for j in 0..size - 1 {
            if num_vec[j] > num_vec[j + 1] {
                let temp = num_vec[j];
                num_vec[j] = num_vec[j + 1];
                num_vec[j + 1] = temp;
            }
        }
    }
    println! {"{:?}",num_vec};
    pause();
}
fn sort_vec_float() {
    print!("> ");
    io::stdout().flush().unwrap();
    let mut vec = String::new();
    io::stdin()
        .read_line(&mut vec)
        .expect("Failed to read line");
    vec.pop();
    vec.pop();
    let vec = &vec[1..];
    let vec: Vec<_> = vec.split(",").collect();
    let mut num_vec = Vec::new();
    for x in vec {
        let n: f32 = FromStr::from_str(x).unwrap();
        num_vec.push(n);
    }
    let size = num_vec.len();
    for _ in 0..size {
        for j in 0..size - 1 {
            if num_vec[j] > num_vec[j + 1] {
                let temp = num_vec[j];
                num_vec[j] = num_vec[j + 1];
                num_vec[j + 1] = temp;
            }
        }
    }
    println! {"{:?}",num_vec};
    pause();
}
fn check_float_100() {
    let mut count_1 = 0;
    let mut count_2 = 0;
    let mut count_3 = 0;
    for _ in 0..100 {
        let num = rand::thread_rng().gen_range(0.0, 900.0);
        if num > 600.0 {
            count_3 += 1;
        } else if num > 300.0 {
            count_2 += 1;
        } else {
            count_1 += 1;
        }
    }
    println!(
        "Above 600: {}\nAbove 300: {}\nAbove 0: {}",
        count_3, count_2, count_1
    );
    pause();
}
fn sort_90() {

    let mut index = 0;
    let mut con_i = 0;
    let mut sum_vec = 0;
    let mut vec: Vec<i32> = Vec::new();
    print!("Before random: ");
    while index <= 10 {

        let mut rng = rand::thread_rng();
        let x = rng.gen_range(0, 100);

        vec.push(x);
        print!("{} ",x);

        while con_i < vec.len(){
            sum_vec += vec[con_i];
            con_i += 1;
        }
        if sum_vec >= 90{
            break;
        }
        index = index + 1;
    }
    println!();
    vec.sort();
    println!("Integer: {:?}", vec);
    pause();
}
fn guessing_game() {
    println!("I'm thinking of a number from 1 to 10.");
    let num: u8 = rand::thread_rng().gen_range(0, 10);
    print!("Your guess: ");
    io::stdout().flush().unwrap();
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    let guess = guess.trim().parse::<u8>().expect("Input a number");
    if guess == num {
        println!("That's right! My secret number was {}!", num);
    } else {
        println!("Sorry, but I was really thinking of {}.", num);
    }
    pause();
}
fn dice() {
    let num_1: u8 = rand::thread_rng().gen_range(0, 6);
    let num_2: u8 = rand::thread_rng().gen_range(0, 6);
    let num = num_1 + num_2;
    println!("Roll #1: {}", num_1);
    println!("Roll #2: {}", num_2);
    println!("The total is {}!", num);
    pause();
}
fn hi_lo() {
    println!("I'm thinking of a number from 1 to 100. You have 7 guesses");
    let num: u8 = rand::thread_rng().gen_range(0, 100);

    for i in 1..8 {
        print!("Guess #{}: ", i);
        io::stdout().flush().unwrap();
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess = guess.trim().parse::<u8>().expect("Input a number");
        if guess == num {
            println!("You guessed it! What are the odds?!?");
            pause();
            return;
        }
        if guess > num {
            println!("Sorry, that guess is too high.");
        }
        if guess < num {
            println!("Sorry, that guess is too low.");
        }
    }
    println!("Sorry, you didn't guess it in 7 tries. You lose");
    pause();
}
fn add_input() {
    let mut total: i32 = 0;
    println!("I will add up the numbers you give me.");
    loop {
        print!("Number: ");
        io::stdout().flush().unwrap();
        let mut num = String::new();
        io::stdin()
            .read_line(&mut num)
            .expect("Failed to read line");
        let num = num.trim().parse::<i32>().expect("Input a number");
        if num == 0 {
            println!("The total so far is: {}", total);
            break;
        }
        total += num;
        println!("The total so far is: {}", total);
    }
    pause();
}
fn calculator() {

    let mut sign: char = '0';
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let mut text = String::new();
        io::stdin()
            .read_line(&mut text)
            .expect("Failed to read line");
        text.pop();
        for x in text.chars() {
            if x == '+' {
                sign = x;

                break;
            } else if x == '-' {
                sign = x;

                break;
            } else if x == '/' {
                sign = x;

                break;
            } else if x == '*' {
                sign = x;
                break;
            } else if x == '^' {
                sign = x;

                break;
            }
        }
        let vec: Vec<_> = text.split(sign).collect();
        let mut num_vec = Vec::new();
        for x in vec {
            let n: u32 = FromStr::from_str(x).unwrap();
            num_vec.push(n);
        }
        if num_vec[0] == 0 {
            println!("Bye, now.");
            pause();
            return;
        }
        if sign == '+' {
            let num = num_vec[0] + num_vec[1];
            println!("{}", num);
        } else if sign == '-' {
            let num = num_vec[0] - num_vec[1];
            println!("{}", num);
        } else if sign == '/' {
            let num = num_vec[0] / num_vec[1];
            println!("{}", num);
        } else if sign == '*' {
            let num = num_vec[0] * num_vec[1];
            println!("{}", num);
        } else if sign == '^' {
            let mut power = num_vec[1];
            let mut num = 1;
            while power != 0 {
                num *= num_vec[0];
                power -= 1;
            }
            println!("{}", num);
        }
    }
}
