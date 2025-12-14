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

pub fn load_img_path(arg: Vec<String>) -> String{
    if arg.len() >= 2{
        let path = arg[1].to_lowercase();
        if img_path_exists(&path) && file_exists(&path) { path }
        else {
            println!("The file '{}' is not a supported image format or file not found.", arg[1]);
            println!("Provide a .PNG, .JPG or .JPEG file.");
            add_img_path()
        }
    } else {
            println!(
                "You didn't open the file using the script. Please enter the path to a .PNG, .JPG, or .JPEG image manually."
            );
            add_img_path()
        }
}

fn add_img_path() -> String {
    //* Get img path from user
    let mut result = String::new();
    stdin()
        .read_line(&mut result)
        .expect("Failed to read line");

    result = result.trim().to_string();

    if img_path_exists(&result) && file_exists(&result) {
        result
    } else {
        println!("Error: file not found");
        add_img_path()
    }
}

fn img_path_exists(path: &str) -> bool {
    let len = path.len();
    
    (len >= 4 && &path[len-4..] == ".png") ||
    (len >= 4 && &path[len-4..] == ".jpg") ||
    (len >= 5 && &path[len-5..] == ".jpeg")
}

fn file_exists(path: &str) -> bool {
    Path::new(path).exists()
}

pub fn value_swap(path: &String, width: &u16, invert: &bool, contrast: &u16, brightness: &u16) -> (String, u16, bool, u16, u16){
    let mut path: String = path.to_string();
    let mut width: u16 = *width;
    let mut invert: bool= *invert;
    let mut contrast: u16= *contrast;
    let mut brightness: u16 = *brightness;


    println!("Choose a setting");
    println!("( 1 ) Image path");
    println!("( 2 ) Width");
    println!("( 3 ) Invert image");
    println!("( 4 ) Contrast level");
    println!("( 5 ) brightness");
    println!("( ENTER ) pass");

    let choise = get_user_input_int("replace_parametrs");

    match choise {
        1 => {
            fn input() -> String {
                let mut path: String = String::new();
                stdin()
                    .read_line(&mut path)
                    .expect("Failed to read line");
                path = path.trim().to_string();

                if file_exists(&path) && img_path_exists(&path) {
                    path = path.trim().to_string()
                } else {
                    println!("Incorrect path, enter again.");
                    input();
                }
            path
            }

            println!("Enter image path [press ENTER to use current path]:");
            let mut new_path: String = String::new();
            stdin()
                .read_line(&mut new_path)
                .expect("Failed to read line");
            new_path = new_path.trim().to_string();
            if new_path.is_empty() { new_path = path}

            if file_exists(&new_path) == false || img_path_exists(&new_path) == false {
                println!("Incorrect path, enter again.");
                input();
            }
            path = new_path.trim().to_string()
        }

        2 => {
            println!("Select width (20 - 400) [default: '80']:");
            width = get_user_input_int("width")
        }
        3 => {
            println!("Invert image? [Y/N] [default: 'N']:");
            invert = get_user_input_bool()
        }
        4 => {
            println!("Set contrast level (Enter your value as x*100, e.g., 15 for 0.15 contrast) [default: '100']:");
            contrast = get_user_input_int("contrast")
        }
        5 => {
            println!("Set brightness (Enter your value as x*100, e.g., 15 for 0.15 brightness, default 100):");
            brightness = get_user_input_int("brightness")
        }
        _ => return (path.to_string(), width, invert, contrast, brightness),
    }

    println!("Change any settings? [Y/N] [default: 'N']:");
    if get_user_input_bool() == true {
        (path, width, invert, contrast, brightness) = value_swap(&path, &width, &invert, &contrast, &brightness)
    }

    (path.to_string(), width, invert, contrast, brightness)
}