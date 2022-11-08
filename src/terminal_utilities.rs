use std::{env::consts};
use std::process::Command;
use terminal_size::{Width, Height, terminal_size};

use crate::structs::TerminalSize;

// Function to clear terminal, useful in windows and linux operating systems
pub fn clear_terminal(){
    if consts::OS == "windows" {
        Command::new("cmd").args(["/C", "cls"]).status().unwrap();
    }
    else{
        Command::new("clear").status().unwrap();
    }
}

// Get the terminal sizes and return inside TerminalSize struct
pub fn get_terminal_sizes() -> Option<TerminalSize>{
    let sizes = terminal_size();
    
    if let Some((Width(w), Height(h))) = sizes{
        return Some(TerminalSize{width: w, height: h});
    }
    else{
        return None;
    }
}