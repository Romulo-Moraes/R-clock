use clap::Parser;
use termcolor::Color;

// Constant string used in clap library, show all available 
// colors for this program
const AVAILABLE_COLORS : &str = "Black\nGray\nBlue\nCyan\nGreen\nMagenta\nRed\nWhite\nYellow\nBrightBlue\nBrightCyan\nBrightGreen\nBrightMagenta\nBrightRed\nBrightWhite\nBrightYellow\n";

// All digits are drawn here, and can be printed by a loop iteration
pub const NUMBER : [[[u8; 3]; 5]; 11] = [
    [
        [1,1,1],
        [1,0,1],
        [1,0,1],
        [1,0,1],
        [1,1,1]
    ],
    [
        [0,0,1],
        [0,0,1],
        [0,0,1],
        [0,0,1],
        [0,0,1]
    ],
    [
        [1,1,1],
        [0,0,1],
        [1,1,1],
        [1,0,0],
        [1,1,1]
    ],
    [
        [1,1,1],
        [0,0,1],
        [1,1,1],
        [0,0,1],
        [1,1,1]
    ],
    [
        [1,0,1],
        [1,0,1],
        [1,1,1],
        [0,0,1],
        [0,0,1]
    ],
    [
        [1,1,1],
        [1,0,0],
        [1,1,1],
        [0,0,1],
        [1,1,1]
    ],
    [
        [1,1,1],
        [1,0,0],
        [1,1,1],
        [1,0,1],
        [1,1,1]
    ],
    [
        [1,1,1],
        [0,0,1],
        [0,0,1],
        [0,0,1],
        [0,0,1]
    ],
    [
        [1,1,1],
        [1,0,1],
        [1,1,1],
        [1,0,1],
        [1,1,1]
    ],
    [
        [1,1,1],
        [1,0,1],
        [1,1,1],
        [0,0,1],
        [1,1,1]
    ],
    [
        [0,0,0],
        [0,1,0],
        [0,0,0],
        [0,1,0],
        [0,0,0]
    ]
];

// Structure that hold the current time
pub struct CurrentTime{
    pub hours : u32,
    pub minutes : u32,
    pub seconds : u32
}

// Structure that will be filled up by terminal_size library
// it is used by the entire program
#[derive(Clone)]
pub struct TerminalSize{
    pub width: u16,
    pub height: u16
}

// Structure used to save some clock paddins that
// will be useful when printing the time in terminal
pub struct ClockPaddings{
    pub top : String,
    pub left : String
}

// Structure used by clap library to provide a argument parsing
#[derive(Parser)]
#[command(author="Rômulo Moraes", version="0.1.0", about="Digital clock", long_about=None)]
pub struct ProgramArguments{
    #[arg(long, short, help=AVAILABLE_COLORS)]
    pub clock_color : Option<String>,

    #[arg(long, short, help=AVAILABLE_COLORS)]
    pub date_color : Option<String>,

    #[arg(long, short, help="Make the clock be small")]
    pub small : bool

}

#[derive(Clone)]
pub struct ClockConfig{
    pub clock_color : Option<Color>,
    pub small_clock : bool,
    pub date_color : Option<Color>
}