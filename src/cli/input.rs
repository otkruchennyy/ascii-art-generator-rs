use std::path::Path;
use std::io::stdin;

pub fn get_user_input_int(msg: &str) -> u16 {
    //* Get an integer from user
    let mut result = String::new();
    stdin()
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
        Ok(num) => match msg {
            "width" => {
                if num < 20 || num > 400 {
                    println!("Width must be between 20 and 400");
                    get_user_input_int(msg)
                } else {
                    num
                }
            }
            "contrast" => {
                if num > 200 {
                    println!("Contrast must be between 0 and 200");
                    get_user_input_int(msg)
                } else {
                    num
                }
            }
            "brightness" => {
                if num > 200 {
                    println!("Brightness must be between 0 and 200");
                    get_user_input_int(msg)
                } else {
                    num
                }
            }
            "replace_parametrs" => {
                if num > 4 {
                    println!("You can choose from 1 to 4");
                    get_user_input_int(msg)
                } else {
                    num
                }
            }
            _ => 0,
        },
        Err(_) => {
            println!("Please enter a valid positive number");
            get_user_input_int(msg)
        }
    };
    result
}

pub fn get_user_input_bool() -> bool {
    //* Get an bool from user
    let mut result = String::new();
    stdin()
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

pub fn add_img_path() -> String {
    //* Get img path from user
    let mut result = String::new();
    stdin()
        .read_line(&mut result)
        .expect("Failed to read line");

    result = result.trim().to_string();

    if file_exists(&result) && img_path_exists(path) {
        result
    } else {
        println!("Error: file not found");
        add_img_path()
    }
}

fn img_path_exists(path: &str) -> bool{
    ( path.len() >= 4 && path[path.len()-4..] == ".png".to_string() ) ||
    ( path[path.len()-4..] == ".jpg".to_string() ) ||
    ( path.len() >= 5 && path[path.len()-5..] == ".jpeg".to_string() )
}

pub fn file_exists(path: &str) -> bool {
    Path::new(path).exists()
}