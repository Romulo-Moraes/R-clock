use std::thread;
use std::io::{stdin, Stdin};
use std::sync::Arc;
use std::sync::{Mutex, MutexGuard};

// Functions that waits for user to press enter to send a signal to main thread with
// the goal of stop the program
pub fn spawn_thread_for_program_stopping(stop_flag : Arc<Mutex<bool>>){
    // Spawn thread
    thread::spawn(move || {
        let program_stdin : Stdin = stdin();
        let mut string_of_stop_trigger : String = String::new();

        // This thread stay stuck in this read line until the user press enter
        program_stdin.read_line(&mut string_of_stop_trigger).unwrap();  
        
        // On the moment that enter has pressed, send a message to main thread
        // through Mutex
        {
            let mut stop_flag_locking : MutexGuard<bool>= stop_flag.lock().unwrap();
            *stop_flag_locking = false;

            drop(stop_flag_locking);
        }
    });
}

