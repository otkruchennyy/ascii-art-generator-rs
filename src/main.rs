mod cli;

use cli::input;

fn img_path_exists(path: &str) -> bool{
    ( path.len() >= 4 && path[path.len()-4..] == ".png".to_string() ) ||
    ( path[path.len()-4..] == ".jpg".to_string() ) ||
    ( path.len() >= 5 && path[path.len()-5..] == ".jpeg".to_string() )
}

fn main() {
    let arg: Vec<String> = std::env::args().collect();
    let input_img_path: String;

    // add path
    if arg.len() >= 2 {
        let path = arg[1].to_lowercase();
        if img_path_exists(&path) {
            input_img_path = path
        } else {
            let path = arg[0].to_lowercase();
            if img_path_exists(&path) {
                    input_img_path = path
                } else {
                    println!(
                        "You didn't open the file using this script, move the image to the terminal window or specify the path to the image yourself."
                    );
                    input_img_path = input::add_img_path();
                }
            }
        } else {
            println!(
                "You didn't open the file using this script, move the image to the terminal window or specify the path to the image yourself."
            );
            input_img_path = input::add_img_path();
        }
    println!("{input_img_path}");
}