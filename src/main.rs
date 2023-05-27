use std::io;

mod control_flow;
mod formatted_print;
mod guessing_game;
mod hello_world;

fn main() {
    loop {
        let mut mode = String::new();
        println!("Select a program to run:");
        io::stdin()
            .read_line(&mut mode)
            .expect("Failed to read line");
        match mode.trim() {
            "hello_world" => {
                hello_world::hello_world();
                hello_world::hello_world_alt();
            }
            "guessing_game" => guessing_game::guessing_game(),
            "std_fmt_general" => formatted_print::std_fmt_general(),
            "std_fmt_display" => formatted_print::std_fmt_display(),
            "std_fmt_list" => formatted_print::std_fmt_list(),
            "std_fmt_formatting" => formatted_print::std_fmt_formatting(),
            _ => {
                println!("Invalid mode");
                continue;
            }
        }
        break;
    }
}
