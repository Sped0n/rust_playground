// activity: celsius fahrenheit converter
pub fn cel_to_fah() {
    println!("Celsius to Fahrenheit converter");
    loop {
        println!("Please input a temperature in Celsius");
        let mut celsius = String::new();
        std::io::stdin()
            .read_line(&mut celsius)
            .expect("Failed to read line");
        let celsius: f64 = match celsius.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please input a number");
                continue;
            }
        };
        let fahrenheit = (celsius * 1.8) + 32.0;
        println!(
            "{:.1} degrees Celsius is {:.1} degrees Fahrenheit",
            celsius, fahrenheit
        );
        break;
    }
}

pub fn fah_to_cel() {
    println!("Fahrenheit to Celsius converter");
    loop {
        println!("Please input a temperature in Fahrenheit");
        let mut fahrenheit = String::new();
        std::io::stdin()
            .read_line(&mut fahrenheit)
            .expect("Failed to read line");
        let fahrenheit: f64 = match fahrenheit.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("please input a number");
                continue;
            }
        };
        let celsius: f64 = (fahrenheit - 32.0) / 1.8;
        println!(
            "{:.1} degrees Fahrenheit is {:.1} degrees Celsius",
            fahrenheit, celsius
        );
        break;
    }
}

// activity: fibonacci
pub fn fibonacci() {
    println!("Generate the nth Fibonacci number");
    loop {
        println!("Please input the index");
        let mut index = String::new();
        std::io::stdin()
            .read_line(&mut index)
            .expect("Failed to read line");
        let index: u64 = match index.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("please input a number");
                continue;
            }
        };
        if index == 0 || index == 1 {
            println!("{0}th fibonacci number is {0}.", index)
        } else {
            let mut i0: u64 = 0;
            let mut i1: u64 = 1;
            let mut i2: u64 = 0;
            for _ in 1..index {
                i2 = i0 + i1;
                i0 = i1;
                i1 = i2;
            }
            println!("{}th fibonacci number is {}", index, i2)
        }
        break;
    }
}

// activity: The Twelve Days of Christmas
pub fn christmas_song() {
    let days = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eigth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];
    let gifts = [
        "A partridge in a pear tree",
        "Two turtle doves",
        "Three french hens",
        "Four calling birds",
        "Five golden rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming",
    ];
    for i in 0..12 {
        println!(
            "On the {} day of Christmas my true love sent to me",
            days[i]
        );
        for j in (0..i + 1).rev() {
            if i == 0 {
                println!("{}", gifts[j]);
            } else if j == 0 {
                println!("And {}", gifts[j]);
            } else {
                println!("{},", gifts[j]);
            }
        }
        println!();
    }
}
