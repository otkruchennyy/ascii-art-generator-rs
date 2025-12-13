fn replace_parametrs(
    //* Override the selected parameter
    mut width_new: u16,
    mut invert_new: bool,
    mut contrast_new: u16,
    mut brightness_new: u16,
) -> (u16, bool, u16, u16) {
    println!("Choose a setting");
    println!("( 1 ) Width");
    println!("( 2 ) Invert image");
    println!("( 3 ) Contrast level");
    println!("( 4 ) brightness");
    println!("( ENTER ) pass");
    let choise = get_user_input_int("replace_parametrs");
    match choise {
        1 => {
            // Width
            println!("Select width (20 - 400) [default: '80']:");
            width_new = get_user_input_int("width");
        }
        2 => {
            // Invert
            println!("Invert image? [Y/N] [default: 'N']:");
            invert_new = get_user_input_bool();
        }
        3 => {
            // Contrast
            println!(
                "Set contrast level (Enter your value as x*100, e.g., 15 for 0.15 contrast) [default: '100']:"
            );
            contrast_new = get_user_input_int("contrast");
        }
        4 => {
            // Brightness
            println!(
                "Set brightness (Enter your value as x*100, e.g., 15 for 0.15 brightness, default 100):"
            );
            brightness_new = get_user_input_int("brightness");
        }
        _ => return (width_new, invert_new, contrast_new, brightness_new),
    }

    println!(
        "Width: {width_new}, invert: {invert_new}, contrast: {contrast_new}, brightness: {brightness_new}"
    );
    println!("Change any settings? [Y/N] [default: 'N']:");

    // Recalling on a Satisfying Condition
    if get_user_input_bool() == true {
        let result: (u16, bool, u16, u16) =
            replace_parametrs(width_new, invert_new, contrast_new, brightness_new);
        result
    } else {
        (width_new, invert_new, contrast_new, brightness_new)
    }
}

