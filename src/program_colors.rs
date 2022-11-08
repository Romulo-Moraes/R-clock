use termcolor::Color;

// Functions that checks if the given color is available
pub fn check_if_color_exists(selected_color : String) -> Option<Color>{

    match selected_color.as_str() {
        "Black" => {
            return Some(Color::Rgb(0, 0, 0));
        },
        "Gray" => {
            return Some(Color::Black);
        }
        "Blue" => { 
            return Some(Color::Blue);
        },
        "BrightBlue" => {
            return Some(Color::Rgb(29, 240, 211));
        }
        ,
        "Cyan" => {
            return Some(Color::Cyan);
        },
        "BrightCyan" => {
            return Some(Color::Rgb(9, 208, 239));
        }
        "Green" => {
            return Some(Color::Green);
        },
        "BrightGreen" => {
            return Some(Color::Rgb(102, 255, 0));
        },
        "Magenta" => {
            return Some(Color::Magenta);
        },
        "BrightMagenta" => {
            return Some(Color::Rgb(255, 0, 205));
        },
        "Red" => {
            return Some(Color::Red);
        },
        "BrightRed" => {
            return Some(Color::Rgb(255, 0, 0));
        },
        "White" => {
            return Some(Color::White);
        },
        "BrightWhite" => {
            return Some(Color::Rgb(255, 255, 255));
        },
        "Yellow" => {
            return Some(Color::Yellow);
        },
        "BrightYellow" => {
            return Some(Color::Rgb(255, 235, 42));
        },
        &_ => {
            // If not, return Option<> with None
            return None;
        }
    }
}