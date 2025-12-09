mod preprocessor;
use std::io;
// use std::io::stdin;
// use std::path::PathBuf;
// use std::path::Path;

fn main() {
    // let img_path: String = "";
    println!("Select width (20 - 400) [default: '80']:");
    let mut width: u16 = get_user_input_int("width");
    println!("Invert image? [Y/N] [default: 'N']:");
    let mut invert: bool = get_user_input_bool();
    println!("Set contrast level (Enter your value as x*100, e.g., 15 for 0.15 contrast) [default: '100']:");
    let mut contrast: u16 = get_user_input_int("contrast");
    println!("Set brightness (Enter your value as x*100, e.g., 15 for 0.15 brightness, default 100):");
    let mut brightness: u16 = get_user_input_int("brightness");

    println!("Width: {width}, invert: {invert}, contrast: {contrast}, brightness: {brightness}");
    println!("Change any settings? [Y/N] [default: 'N']:");
    
    if get_user_input_bool() == true {
        (width, invert, contrast, brightness) = 
        replace_parametrs(width, invert, contrast, brightness);
    }
    println!("Width: {width}, invert: {invert}, contrast: {contrast}, brightness: {brightness}");
}

fn replace_parametrs(mut width_new: u16, mut invert_new: bool, mut contrast_new: u16, mut brightness_new: u16) -> (u16, bool, u16, u16) {
    println!("Choose a setting");
    println!("( 1 ) Width");
    println!("( 2 ) Invert image");
    println!("( 3 ) Contrast level");
    println!("( 4 ) brightness");
    println!("( ENTER ) pass");
    let choise = get_user_input_int("replace_parametrs");
    match choise {
        1 => {
            println!("Select width (20 - 400) [default: '80']:");
            width_new = get_user_input_int("width");
        }
        2 => {
            println!("Invert image? [Y/N] [default: 'N']:");
            invert_new = get_user_input_bool();
        }
        3 => {
            println!("Set contrast level (Enter your value as x*100, e.g., 15 for 0.15 contrast) [default: '100']:");
            contrast_new = get_user_input_int("contrast");
        }
        4 => {
            println!("Set brightness (Enter your value as x*100, e.g., 15 for 0.15 brightness, default 100):");
            brightness_new = get_user_input_int("brightness");
        }
        _ => return (width_new, invert_new, contrast_new, brightness_new),
    }
    
    println!("Width: {width_new}, invert: {invert_new}, contrast: {contrast_new}, brightness: {brightness_new}");
    println!("Change any settings? [Y/N] [default: 'N']:");

    if get_user_input_bool() == true {
        let result: (u16, bool, u16, u16) = replace_parametrs(width_new, invert_new, contrast_new, brightness_new);
        result
    } else { 
        (width_new, invert_new, contrast_new, brightness_new)
    }
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
            "replace_parametrs" => return 0,
            _ => panic!("Eroor: msg not found"),
        }
    }

    // Error handling
    let result: u16 = match result.trim().parse::<u16>() {
        Ok(num) => {
            match msg {
                "width" => {
                    if num < 20 || num > 400 {
                    println!("Width must be between 20 and 400");
                    get_user_input_int(msg)
                    } else { num }
                }
                "contrast" => {
                    if num > 200 {
                        println!("Contrast must be between 0 and 200");
                        get_user_input_int(msg)
                    } else { num }
                }
                "brightness" => {
                    if num > 200 {
                        println!("Brightness must be between 0 and 200");
                        get_user_input_int(msg)
                    } else { num }
                }
                "replace_parametrs" => {
                    if num > 4 {
                        println!("You can choose from 1 to 4");
                        get_user_input_int(msg)
                    } else { num }
                }
                _ => 0,
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
            println!("Please enter Y or N: ");
            get_user_input_bool()
        }
    }
}