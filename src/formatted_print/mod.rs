use std::fmt::{Display, Formatter, Result};

pub fn std_fmt_general() {
    // In general, the `{}` will be automatically replaced with any
    // arguments. These will be stringified.
    println!("{} days", 31i64);

    // Without a suffix, 31 becomes an i32. You can change what type 31 is
    // by providing a suffix. The number 31i64 for example has the type i64.

    // There are various optional patterns this works with. Positional
    // arguments can be used.
    println!("{0}, this is {1}. {1}, this is {0}.", "Alice", "Bob");

    // As can named arguments.
    println!(
        "{subject} {verb} {object}.",
        object = "the lazy dog",
        subject = "the quick brown fox",
        verb = "jumps over"
    );

    // Special formatting can be specified after a `:`.
    // below is an example of binary formatting.
    println!(
        "{} of {:b} people know binary, the other half doesn't.",
        1, 2
    );

    // You can right-align text with a specified width. This will output
    // "     1". 5 white spaces and a "1".
    println!("{number:>width$}", number = 1, width = 6);

    // You can pad numbers with extra zeroes. This will output "000001".
    println!("{number:>0width$}", number = 1, width = 6);

    // james bond example
    println!("My name is {1}, {0} {1}.", "James", "Bond");

    // pi example
    let pi = std::f64::consts::PI;
    println!("Pi is roughly {:.3}.", pi);
}

// A structure holding two numbers. `Debug` will be derived so the results can
// be contrasted with `Display`.
#[derive(Debug)]
struct MinMax(i64, i64);

// Implement `Display` for `MinMax`.
impl Display for MinMax {
    fn fmt(&self, f: &mut Formatter) -> Result {
        // Use `self.number` to refer to each positional data point.
        write!(f, "({}, {})", self.0, self.1)
    }
}

// Define a structure where the fields are nameable for comparison.
#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64,
}
// similarly, implement `Display` for `Point2D`
impl Display for Point2D {
    fn fmt(&self, f: &mut Formatter) -> Result {
        // Customize so only `x` and `y` are denoted.
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

// activity: Complex struct
#[derive(Debug)]
struct Complex {
    real: f64,
    imag: f64,
}
impl Display for Complex {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{} + {}i", self.real, self.imag)
    }
}

pub fn std_fmt_display() {
    let minmax = MinMax(0, 14);
    println!("Compare structures:");
    println!("Display: {}", minmax);
    println!("Debug: {:?}", minmax);

    let big_range = MinMax(-300, 300);
    let small_range = MinMax(-3, 3);
    println!(
        "The big range is {big} and the small is {small}.",
        small = small_range,
        big = big_range
    );

    let point = Point2D { x: 3.3, y: 7.2 };
    println!("Compare points:");
    println!("Display: {}", point);
    println!("Debug: {:?}", point);

    // activity: Complex struct
    let complex = Complex {
        real: 3.3,
        imag: 7.2,
    };
    println!("Compare complex:");
    println!("Display: {}", complex);
    println!("Debug: {:?}", complex);
}

// activity: List struct
struct List(Vec<i32>);

impl Display for List {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let vec = &self.0;

        // write opening bracket
        write!(f, "[")?;

        for (count, v) in vec.iter().enumerate() {
            // add comma after each element except the first
            if count != 0 {
                write!(f, ", ")?;
            }
            // add index output
            write!(f, "{}: {}", count, v)?;
        }

        // write closing bracket
        write!(f, "]")
    }
}

pub fn std_fmt_list() {
    let v = List(vec![1, 2, 3]);
    println!("{}", v);
}

// activity: City struct
struct City {
    name: &'static str,
    // Latitude
    lat: f32,
    // Longitude
    lon: f32,
}

impl Display for City {
    // `f` is a buffer, and this method must write the formatted string into it
    fn fmt(&self, f: &mut Formatter) -> Result {
        let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
        let lon_c = if self.lon >= 0.0 { 'E' } else { 'W' };

        // `write!` is like `format!`, but it will write the formatted string
        // into a buffer (the first argument)
        write!(
            f,
            "{}: {:.3}°{} {:.3}°{}",
            self.name,
            self.lat.abs(),
            lat_c,
            self.lon.abs(),
            lon_c
        )
    }
}

// activity: Color struct
#[derive(Debug)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "RGB ({}, {}, {}) 0x{:02X}{:02X}{:02X}",
            self.red, self.green, self.blue, self.red, self.green, self.blue
        )
    }
}

pub fn std_fmt_formatting() {
    for city in [
        City {
            name: "Dublin",
            lat: 53.347778,
            lon: -6.259722,
        },
        City {
            name: "Oslo",
            lat: 59.95,
            lon: 10.75,
        },
        City {
            name: "Vancouver",
            lat: 49.25,
            lon: -123.1,
        },
    ]
    .iter()
    {
        println!("{}", *city);
    }
    for color in [
        Color {
            red: 128,
            green: 255,
            blue: 90,
        },
        Color {
            red: 0,
            green: 3,
            blue: 254,
        },
        Color {
            red: 0,
            green: 0,
            blue: 0,
        },
    ]
    .iter()
    {
        println!("{}", *color);
    }
}
