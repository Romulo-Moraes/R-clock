// Program modules
mod structs;
mod program_configuration;
mod terminal_utilities;
mod clock;
mod program_traits;
mod program_colors;
mod helpers;

// Imports
use std::{sync::{Mutex,MutexGuard ,Arc}, io::{stdout, Stdout}};
use std::thread::sleep;
use std::time::Duration;
use chrono::{DateTime, Local};
use clap::Parser;
use crossterm::{cursor::{MoveTo, Hide, Show}, ExecutableCommand};
use clock::{draw_digital_clock_in_terminal, extract_time_from_date_time_struct};
use helpers::handle_program_arguments;
use structs::{CurrentTime, TerminalSize, ProgramArguments, ClockConfig};
use terminal_utilities::{get_terminal_sizes, clear_terminal};

fn main() {
    // Variable declarations
    let program_stop_flag : Arc<Mutex<bool>> = Arc::new(Mutex::new(true));
    let mut stop_flag_to_be_use_in_loop : bool = true;
    let mut program_stop_flag_mutex_guard : MutexGuard<bool>;
    let mut local_time : DateTime<Local> = Local::now();
    let mut local_time_splited_in_hours_minutes_and_seconds: CurrentTime;
    let mut screen_sizes : TerminalSize = get_terminal_sizes().unwrap();
    let mut screen_sizes_for_testing : TerminalSize = get_terminal_sizes().unwrap();
    let mut program_stdout : Stdout = stdout();
    let program_argument : ProgramArguments = ProgramArguments::parse();
    let clock_config : ClockConfig = handle_program_arguments(program_argument);

    // This thread will set a boolean inside an Arc<Mutex<>> when the user press enter
    // When this happens the program should stop clock the clock, show cursor and clear terminal  
    program_configuration::spawn_thread_for_program_stopping(program_stop_flag.clone());

    //Clear terminal for first execution, avoiding some junk characters 
    // in terminal that was written before the program execution
    clear_terminal();

    // Hide the cursor for a better experience
    program_stdout.execute(Hide).unwrap();

    // Main structure of program, each second the clock update its value
    while stop_flag_to_be_use_in_loop == true{
        local_time_splited_in_hours_minutes_and_seconds = extract_time_from_date_time_struct(&local_time);

        // If the size of terminal changed on mid way of the program, clear it.
        // it can avoid some bugs, like ghost numbers on the place that before was
        // a real number of the clock
        if screen_sizes.width != screen_sizes_for_testing.width || screen_sizes.height != screen_sizes_for_testing.height{
            screen_sizes = screen_sizes_for_testing;
            clear_terminal();
        }

        // For each second, this command will put the cursor at (0,0)
        // in terminal screen, allowing to print the entire clock again
        // over and over again
        program_stdout.execute(MoveTo(0,0)).unwrap();

        // Checking if the flag that represents the program life still true.
        // Everytime that the user press enter, the flag will change to false
        {
            program_stop_flag_mutex_guard = program_stop_flag.lock().unwrap();
            if *program_stop_flag_mutex_guard == false {
                // If the user wan't kill the program, then set this variable to
                // false. This same variable is being used in the main while statement of
                // the program.
                stop_flag_to_be_use_in_loop = false;
            }

            drop(program_stop_flag_mutex_guard);
        }

        program_stdout.execute(MoveTo(0,0)).unwrap();

        // Draw the clock
        draw_digital_clock_in_terminal(local_time_splited_in_hours_minutes_and_seconds, screen_sizes.clone(), clock_config.clone(), &local_time);
        
        // If the program need exit, then this code block can be useful.
        // Usually, even if the program need to exit right now it would must
        // wait more one second, with this if case the situation can be handled
        // and wait just if the enter key was not pressed
        if stop_flag_to_be_use_in_loop == true{
            sleep(Duration::from_secs(1));
        }

        // Get the current terminal size for testing in next while iteration
        screen_sizes_for_testing = get_terminal_sizes().unwrap();

        // Append one second in time
        local_time = local_time.checked_add_signed(chrono::Duration::seconds(1)).unwrap();
    }

    // This part of only is executed if the user pressed enter.
    // Cleaning terminal and showing the cursor again...
    terminal_utilities::clear_terminal();
    program_stdout.execute(Show).unwrap();
}
