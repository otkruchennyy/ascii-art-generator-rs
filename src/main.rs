mod cli;

use cli::input;

const LINE: &str = "- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -";
fn line() {
    println!("{LINE}");
}
fn space() {
    println!("\n");
}

fn main() {
    // input
    let arg: Vec<String> = std::env::args().collect();
    let mut img_path: String = input::load_img_path(arg);
    
    println!("Select width (20 - 400) [default: '80']:");
    let mut width: u16 = input::get_user_input_int("width");

    println!("Invert image? [Y/N] [default: 'N']:");
    let mut invert: bool = input::get_user_input_bool();

    println!("Set contrast level (Enter your value as x*100, e.g., 15 for 0.15 contrast) [default: '100']:");
    let mut contrast: u16 = input::get_user_input_int("contrast");

    println!("Set brightness (Enter your value as x*100, e.g., 15 for 0.15 brightness, default 100):");
    let mut brightness: u16 = input::get_user_input_int("brightness");

    space();
    line();
    // replacement input
    println!("Image path: {img_path}");
    println!("Width: {width}");
    println!("invert: {invert}");
    println!("contrast: {contrast}");
    println!("brightness: {brightness}");
    line();
    space();

    // solution werification
    println!("Change any settings? [Y/N] [default: 'N']:");
    if input::get_user_input_bool() == true {
        (img_path, width, invert, contrast, brightness) = input::value_swap(&img_path, &width, &invert, &contrast, &brightness)
    }
    line();
    println!("Image path: {img_path}");
    println!("Width: {width}");
    println!("invert: {invert}");
    println!("contrast: {contrast}");
    println!("brightness: {brightness}");
    line();
}