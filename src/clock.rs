use std::{io::{stdout, Write}};

use chrono::{prelude::Timelike, DateTime, Local};

use crate::{
    helpers::print_message_of_not_enough_space_for_rendering,
    program_traits::StringTrait,
    structs::{
        ClockConfig, ClockPaddings, CurrentTime, TerminalSize, NUMBER,
        RAINBOW_STRUCT_FOR_CLOCK, RAINBOW_STRUCT_FOR_DATE,
    },
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

pub fn print_date(
    screen_size: TerminalSize,
    date: &DateTime<Local>,
    date_color: Option<Color>,
    rainbow_mode_for_date: bool,
) {
    // Necessary variables
    let mut colorful_stdout: StandardStream = StandardStream::stdout(ColorChoice::Always);
    let mut date_left_padding: String = String::new();
    let formatted_data = date.format("%A, %b %d %Y").to_string();
    let formatted_data_length : usize;
    let mut colors_index : usize = 0;

    // Add the necessary padding to a variable that will be printed before the date
    date_left_padding.append_char_specified_times(
        ' ',
        screen_size.width / 2 - (formatted_data.to_string().len() as u16 / 2),
    );

    // If this comparision is true, it means that the color of date is a solid color
    if rainbow_mode_for_date == false {
        // Checking if some colors was set for date
        match date_color {
            Some(selected_color) => {
                // If yes, then set output color.
                colorful_stdout
                    .set_color(ColorSpec::new().set_fg(Some(selected_color)))
                    .unwrap();
            }
            None => {
                // else, then use a default color
                colorful_stdout
                    .set_color(ColorSpec::new().set_fg(Some(Color::Blue)))
                    .unwrap();
            }
        }

        // Print the formatted data with some padding before it
        println!("{}{}", date_left_padding, formatted_data);
    }
    // Else, it is in rainbow mode
    else{
        formatted_data_length = formatted_data.len();

        // Print the padding first
        print!("{}", date_left_padding);

        // From each character in string, print it with a different color
        for x in 0..formatted_data_length{
            colorful_stdout.set_color(ColorSpec::new().set_fg(Some(RAINBOW_STRUCT_FOR_DATE[colors_index]))).unwrap();
            print!("{}", formatted_data.chars().nth(x).unwrap());
            colors_index += 1;

            if colors_index >= 7 {
                colors_index = 0;
            }
        }

        // Flush with \n
        println!("");
    }

    colorful_stdout.reset().unwrap();
}

pub fn extract_time_from_date_time_struct(time: &DateTime<Local>) -> CurrentTime {
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
    local_time: &DateTime<Local>,
) {
    let mut clock_sections: Vec<Vec<u32>> = Vec::new();
    let mut i: usize = 0;
    let mut j: usize = 0;
    let mut color_output: StandardStream = StandardStream::stdout(ColorChoice::Always);
    let paddings: ClockPaddings;
    let mut exists_space_enough_to_render_digital_clock: bool = true;
    let mut rainbow_index = 0;

    // Checking if the current terminal size supports the selected clock horizontally

    // For small version
    if clock_configuration.small_clock == true {
        if screen_sizes.width < 45 {
            print_message_of_not_enough_space_for_rendering(true);
            exists_space_enough_to_render_digital_clock = false;
        }
    }
    // For normal version
    else {
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

        // Add the time to a Vector, this will be useful to iterate and print
        // all elements inside NUMBER constant
        clock_sections.push(split_digits_of_clock_section(time.hours));
        clock_sections.push(vec![10]);
        clock_sections.push(split_digits_of_clock_section(time.minutes));

        // Just append these values if the clock isn't small
        if clock_configuration.small_clock == false {
            clock_sections.push(vec![10]);
            clock_sections.push(split_digits_of_clock_section(time.seconds));
        }

        // Print the top padding
        print!("{}", paddings.top);

        // While from i < 5, means the count of line that the clock has.
        while i < 5 {
            // Print the clock's left padding for each line
            print!("{}", paddings.left);

            // For each section...
            for section in clock_sections.iter() {

                // For each digit in section...
                for digit in section {

                    // Each number has 3 leds of width, then
                    // this while runs to print each one.
                    while j < 3 {

                        // If this comparision is true, it means that the led should be turned on
                        if NUMBER[*digit as usize][i][j] == 1 {

                            // Checking if the rainbow mode isn't on
                            if clock_configuration.rainbow_mode_for_clock == 0 {

                                // If not, then get the clock color that the user passed to program
                                match clock_configuration.clock_color {
                                    Some(selected_color) => {
                                        color_output
                                            .set_color(
                                                ColorSpec::new().set_bg(Some(selected_color)),
                                            )
                                            .unwrap();
                                    }
                                    None => {
                                        // If there's no color, then set a default color
                                        color_output
                                            .set_color(ColorSpec::new().set_bg(Some(Color::Blue)))
                                            .unwrap();
                                    }
                                }
                            } else {
                                // Here the rainbow mode is on.

                                // Exists two modes of rainbow, one that make a vertical rainbow
                                // the other that make a horizontal rainbow. they increment the 
                                // rainbow_index variable in different times
                                color_output
                                    .set_color(
                                        ColorSpec::new()
                                            .set_bg(Some(RAINBOW_STRUCT_FOR_CLOCK[rainbow_index])),
                                    )
                                    .unwrap();
                            }
                        } else {
                            color_output.reset().unwrap();
                        }

                        print!("  ");
                        j += 1;

                        // This is for rainbow 1, rainbow in vertical mode,
                        // each led printed the variable must add 1 to reach
                        // others colors in array
                        if clock_configuration.rainbow_mode_for_clock == 1 {
                            rainbow_index += 1;

                            if rainbow_index >= 7 {
                                rainbow_index = 0;
                            }
                        }
                    }

                    color_output.reset().unwrap();
                    print!("  ");
                    j = 0;
                }
            }
            color_output.reset().unwrap();
            stdout().flush().unwrap();
            println!("");


            if clock_configuration.rainbow_mode_for_clock == 1 {
                // The rainbow mode 1 must back to zero after a printed line
                // for the same color printed above be printed now
                rainbow_index = 0;
            } else {
                // But the rainbow mode 2 just increments one, because
                // it needs to print a different color in each line
                rainbow_index += 1;
            }
            i += 1;
        }

        println!("");

        print_date(
            screen_sizes.clone(),
            &local_time,
            clock_configuration.date_color,
            clock_configuration.rainbow_mode_for_date,
        );
    }
}
