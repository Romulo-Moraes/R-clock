use std::process::exit;

use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

use crate::{
    program_colors::check_if_color_exists,
    structs::{ClockConfig, ProgramArguments},
    terminal_utilities::clear_terminal,
};

pub fn print_message_of_not_enough_space_for_rendering(horizontally_troubles: bool) {
    let mut colorful_stdout: StandardStream = StandardStream::stdout(ColorChoice::Always);

    colorful_stdout
        .set_color(ColorSpec::new().set_fg(Some(Color::Red)))
        .unwrap();

    clear_terminal();
    println!(
        "There's no space enough to render the digital clock in {}.",
        if horizontally_troubles == true {
            "horizontally"
        } else {
            "vertically"
        }
    );
    colorful_stdout.reset().unwrap();
}

fn print_message_about_no_existence_of_selected_color() {
    let mut colorful_output: StandardStream = StandardStream::stdout(ColorChoice::Always);

    colorful_output
        .set_color(ColorSpec::new().set_fg(Some(Color::Red)))
        .unwrap();
    println!("The selected color not exists. try again.");
    colorful_output.reset().unwrap();
    exit(0);
}

pub fn handle_program_arguments(program_arguments: ProgramArguments) -> ClockConfig {
    let mut clock_color: Option<Color> = None;
    let mut date_color: Option<Color> = None;

    // Checking if the option of clock color was triggered
    match program_arguments.clock_color {
        Some(the_color) => {
            // If the code reaches here, that means that the option was trigerred

            // Checking if this color is available
            clock_color = check_if_color_exists(the_color);
            match clock_color {
                // If yes, there's nothing more todo, the color is already saved
                Some(_) => {}
                // If not, give a coloful message error
                None => {
                    print_message_about_no_existence_of_selected_color();
                }
            }
        }
        None => {}
    }

    // Same thing here
    match program_arguments.date_color {
        Some(the_color) => {

            date_color = check_if_color_exists(the_color);
            match date_color {
                Some(_) => {}
                None => {
                    print_message_about_no_existence_of_selected_color();
                }
            }
        }
        None => {}
    }

    // Return results
    return ClockConfig {
        clock_color: clock_color,
        small_clock: program_arguments.small,
        date_color: date_color
    };
}
