use std::process::exit;

use chrono::{DateTime, Datelike, FixedOffset, Local, NaiveDate, Timelike};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

use crate::{
    program_colors::check_if_color_exists,
    structs::{ClockConfig, ProgramArguments},
    terminal_utilities::clear_terminal,
};

pub fn fill_local_time_with_date_and_time(clock_config: &ClockConfig) -> DateTime<Local> {
    // Getting a default date and time
    let current_time: DateTime<Local> = Local::now();
    let (hour, minute, second, day, month, year): (u32, u32, u32, u32, u32, i32);

    // Checking if user set a custom hour
    if let Some(new_hour) = clock_config.hour {
        hour = new_hour;
    } else {
        hour = current_time.hour();
    }

    // Checking if user set a custom minute
    if let Some(new_minute) = clock_config.minute{
        minute = new_minute;
    }
    else{
        minute = current_time.minute();
    }

    // Checking if user set a custom second
    if let Some(new_second) = clock_config.second{
        second = new_second;
    }
    else{
        second = current_time.second();
    }

    // Checking if user set a custom day
    if let Some(new_day) = clock_config.day{
        day = new_day;
    }
    else{
        day = current_time.day();
    }

    // Checking if user set a custom month
    if let Some(new_month) = clock_config.month{
        month = new_month;
    }
    else{
        month = current_time.month();
    }

    // Checking if user set a custom year
    if let Some(new_year) = clock_config.year{
        year = new_year;
    }
    else{
        year = current_time.year();
    }

    // Getting the custom year, month and day and checking if nothing is wrong
    if let Some(year_month_and_year) = NaiveDate::from_ymd_opt(year, month, day) {

        // Getting custom hour, minute and second and checkingif nothing is wrong
        if let Some(complete_date_time) = year_month_and_year.and_hms_opt(hour, minute, second){

            // Here both of previous steps weren't wrong, then return the modified date and time
            return DateTime::from_local(complete_date_time,
                FixedOffset::east(0),
            );
        }
        else{
            // If something is wrong with hour, minute or second, then write the message in red color and exit
            print_alert_message_with_red_color_and_exit("Something is wrong in the passed hours, minutes or seconds. try again");
            exit(0);
        }
    }
    else{
        // If something is wrong with year, month or day, then write the message in red color and exit
        print_alert_message_with_red_color_and_exit("Something is wrong in the passed day, month or year. try again");
        exit(0);
    }

    
}

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

fn print_alert_message_with_red_color_and_exit(message : &str) {
    let mut colorful_output: StandardStream = StandardStream::stdout(ColorChoice::Always);

    colorful_output
        .set_color(ColorSpec::new().set_fg(Some(Color::Red)))
        .unwrap();

    println!("{}", message);
    colorful_output.reset().unwrap();
    exit(0);
}

pub fn handle_program_arguments(program_arguments: ProgramArguments) -> ClockConfig {
    let mut clock_color: Option<Color> = None;
    let mut date_color: Option<Color> = None;
    let mut rainbow_mode_for_clock: u8 = 0;
    let mut rainbow_mode_for_date: bool = false;

    // Checking if the option of clock color was triggered
    match program_arguments.clock_color {
        Some(the_color) => {
            // If the code reaches here, that means that the option was trigerred

            // Checking if this color is available
            match the_color.as_str() {
                "Rainbow" => {
                    rainbow_mode_for_clock = 1;
                }
                "Rainbow2" => {
                    rainbow_mode_for_clock = 2;
                }
                &_ => {
                    clock_color = check_if_color_exists(the_color);
                    match clock_color {
                        // If yes, there's nothing more todo, the color is already saved
                        Some(_) => {}
                        // If not, give a coloful message error
                        None => {
                            print_alert_message_with_red_color_and_exit("The selected color not exists. try again.");
                        }
                    }
                }
            }
        }
        None => {}
    }

    // Same thing here
    match program_arguments.date_color {
        Some(the_color) => {
            if the_color == "Rainbow" {
                rainbow_mode_for_date = true;
            } else {
                date_color = check_if_color_exists(the_color);
                match date_color {
                    Some(_) => {}
                    None => {
                        print_alert_message_with_red_color_and_exit("The selected color not exists. try again.");
                    }
                }
            }
        }
        None => {}
    }

    // Return results
    return ClockConfig {
        clock_color: clock_color,
        small_clock: program_arguments.small,
        date_color: date_color,
        rainbow_mode_for_clock: rainbow_mode_for_clock,
        rainbow_mode_for_date: rainbow_mode_for_date,
        hour: program_arguments.hour,
        minute: program_arguments.minute,
        second: program_arguments.second,
        day: program_arguments.day,
        month: program_arguments.month,
        year: program_arguments.year,
    };
}
