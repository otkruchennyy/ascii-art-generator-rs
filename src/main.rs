mod preprocessor;
use std::io;
// use std::io::stdin;
// use std::path::PathBuf;
// use std::path::Path;

fn main() {
    println!("Select width (20 - 400, default 80):");
    let width = get_user_input_int("width");
    println!("Invert image? [Y/N]:");
    let invert: bool = get_user_input_bool();
    println!("Set contrast level (Enter your value as x*100, e.g., 15 for 0.15 contrast, default 100):");
    let contrast: u16 = get_user_input_int("contrast");
    println!("Set brightness (Enter your value as x*100, e.g., 15 for 0.15 brightness, default 100):");
    let brightness: u16 = get_user_input_int("brightness");

    // println!("{width}, {invert}, {contrast}");

}

fn get_user_input_int(msg: &str) -> u16 {
    let mut result = String::new();
    io::stdin()
        .read_line(&mut result)
        .expect("Failed to read line");

    if result.trim().is_empty() {
        match msg {
            "width" => return 80,
            "contrast" => return 100,
            "brightness" => return 100,
            _ => panic!("Eroor: msg not found"),
        }
    }

    // Error handling
    let result: u16 = match result.trim().parse::<u16>() {
        Ok(num) => {
            match msg {
                "width" if num < 20 || num > 400 => {
                    println!("Width must be between 20 and 400");
                    get_user_input_int(msg)
                }
                "contrast" if num > 200 => {
                    println!("Сontrast must be between 0 and 200");
                    get_user_input_int(msg)
                }
                "brightness" if num > 200 => {
                    println!("Brightness must be between 0 and 200");
                    get_user_input_int(msg)
                }
                _ => 100,
            }
        }
        Err(_) => {
            println!("Please enter a valid positive number");
            get_user_input_int(msg)
        }
    };
    result
}

fn get_user_input_bool() -> bool {
    let mut result = String::new();
    io::stdin()
        .read_line(&mut result)
        .expect("Failed to read line");

    if result.trim().is_empty() {
        return false;
    }

    match result.trim() {
        "Y" | "y" => true,
        "N" | "n" => false,
        _ => {
            print!("Please enter Y or N: ");
            get_user_input_bool()
        }
    }
}