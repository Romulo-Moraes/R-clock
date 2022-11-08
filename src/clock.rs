use std::io::{stdout, Write};

use chrono::{prelude::Timelike, DateTime, Local};

use crate::{
    program_traits::StringTrait,
    structs::{ClockConfig, ClockPaddings, CurrentTime, TerminalSize, NUMBER}, helpers::print_message_of_not_enough_space_for_rendering,
};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

fn get_clock_padding(screen_sizes: &TerminalSize, small_digital_clock: bool) -> ClockPaddings {
    let mut top_padding: String = String::new();
    let mut left_padding: String = String::new();
    let save_left_padding_calculation: u16;
    let save_top_padding_calculation: u16;

    save_top_padding_calculation = screen_sizes.height / 2 - 2;

    if small_digital_clock == true {
        save_left_padding_calculation = screen_sizes.width / 2 - 19;
    } else {
        save_left_padding_calculation = screen_sizes.width / 2 - 31;
    }

    top_padding.append_char_specified_times('\n', save_top_padding_calculation);
    left_padding.append_char_specified_times(' ', save_left_padding_calculation);

    return ClockPaddings {
        top: top_padding,
        left: left_padding,
    };
}

fn split_digits_of_clock_section(value: u32) -> Vec<u32> {
    let mut digits: Vec<u32> = Vec::new();

    if value < 10 {
        digits.push(0);
        digits.push(value);
    } else {
        digits.push(value / 10);
        digits.push(value % 10);
    }

    return digits;
}

pub fn print_date(screen_size : TerminalSize, date : &DateTime<Local>, date_color : Option<Color>){
    let mut colorful_stdout : StandardStream = StandardStream::stdout(ColorChoice::Always);
    let mut date_left_padding : String = String::new();
    let formatted_data = date.format("%A, %b %d %Y");

    // Checking if some colors was set for date
    match date_color {
        Some(selected_color) => {
            // If yes, then set output color.
            colorful_stdout.set_color(ColorSpec::new().set_fg(Some(selected_color))).unwrap();
        },
        None => {
            // else, then use a default color
            colorful_stdout.set_color(ColorSpec::new().set_fg(Some(Color::Blue))).unwrap();
        }
    }
    date_left_padding.append_char_specified_times(' ', screen_size.width / 2 - (formatted_data.to_string().len() as u16 / 2));

    println!("{}{}", date_left_padding, formatted_data);
    colorful_stdout.reset().unwrap();
}

pub fn extract_time_from_date_time_struct(time : &DateTime<Local>) -> CurrentTime {

    return CurrentTime {
        hours: time.hour(),
        minutes: time.minute(),
        seconds: time.second(),
    };
}

pub fn draw_digital_clock_in_terminal(
    time: CurrentTime,
    screen_sizes: TerminalSize,
    clock_configuration: ClockConfig,
    local_time : &DateTime<Local>
) {
    let mut clock_sections: Vec<Vec<u32>> = Vec::new();
    let mut i: usize = 0;
    let mut j: usize = 0;
    let mut color_output: StandardStream = StandardStream::stdout(ColorChoice::Always);
    let paddings: ClockPaddings;
    let mut exists_space_enough_to_render_digital_clock: bool = true;

    // Checking if the current terminal size supports the selected clock horizontally

    // For small version
    if clock_configuration.small_clock == true {
        if screen_sizes.width < 45 {
            print_message_of_not_enough_space_for_rendering(true);
            exists_space_enough_to_render_digital_clock = false;
        }
    }
    // For normal version
    else{
        if screen_sizes.width < 65 {
            print_message_of_not_enough_space_for_rendering(true);
            exists_space_enough_to_render_digital_clock = false;
        }
    }

    // Checking if the current terminal size supports the selected clock vertically

    if screen_sizes.height < 13 {
        print_message_of_not_enough_space_for_rendering(false);
        exists_space_enough_to_render_digital_clock = false;
    }

    if exists_space_enough_to_render_digital_clock == true {
        paddings = get_clock_padding(&screen_sizes, clock_configuration.small_clock);

        clock_sections.push(split_digits_of_clock_section(time.hours));
        clock_sections.push(vec![10]);
        clock_sections.push(split_digits_of_clock_section(time.minutes));

        if clock_configuration.small_clock == false {
            clock_sections.push(vec![10]);
            clock_sections.push(split_digits_of_clock_section(time.seconds));
        }

        print!("{}", paddings.top);

        while i < 5 {
            print!("{}", paddings.left);
            for section in clock_sections.iter() {
                for digit in section {
                    while j < 3 {
                        if NUMBER[*digit as usize][i][j] == 1 {
                            match clock_configuration.clock_color {
                                Some(selected_color) => {
                                    color_output
                                        .set_color(ColorSpec::new().set_bg(Some(selected_color)))
                                        .unwrap();
                                }
                                None => {
                                    color_output
                                        .set_color(ColorSpec::new().set_bg(Some(Color::Blue)))
                                        .unwrap();
                                }
                            }
                        } else {
                            color_output.reset().unwrap();
                        }

                        print!("  ");
                        j += 1;
                    }

                    color_output.reset().unwrap();
                    print!("  ");
                    j = 0;
                }
            }
            color_output.reset().unwrap();
            stdout().flush().unwrap();
            println!("");

            i += 1;
        }

        println!("");


        print_date(screen_sizes.clone(), &local_time, clock_configuration.date_color);
    }
}
