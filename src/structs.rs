use clap::Parser;

use termcolor::{Color};

// Constant string used in clap library, show all available 
// colors for this program
const AVAILABLE_COLORS_FOR_CLOCK : &str = "Black\nGray\nBlue\nCyan\nGreen\nMagenta\nRed\nWhite\nYellow\nBrightBlue\nBrightCyan\nBrightGreen\nBrightMagenta\nBrightRed\nBrightWhite\nBrightYellow\nRainbow\nRainbow2\n";
const AVAILABLE_COLORS_FOR_DATE : &str = "Black\nGray\nBlue\nCyan\nGreen\nMagenta\nRed\nWhite\nYellow\nBrightBlue\nBrightCyan\nBrightGreen\nBrightMagenta\nBrightRed\nBrightWhite\nBrightYellow\nRainbow\n";

const RAINBOW_RED_FOR_CLOCK : Color = Color::Rgb(150, 0, 0);
const RAINBOW_ORANGE_FOR_CLOCK : Color = Color::Rgb(255, 127, 0);
const RAINBOW_YELLOW_FOR_CLOCK : Color = Color::Rgb(255, 255, 0);
const RAINBOW_GREEN_FOR_CLOCK : Color = Color::Rgb(0, 150, 0);
const RAINBOW_BLUE_FOR_CLOCK : Color = Color::Rgb(0, 0, 255);
const RAINBOW_INDIGO_FOR_CLOCK : Color = Color::Rgb(75, 0, 130);
const RAINBOW_VIOLET_FOR_CLOCK : Color = Color::Rgb(148, 0, 211);

const RAINBOW_RED_FOR_DATE : Color = Color::Rgb(255, 0, 0);
const RAINBOW_ORANGE_FOR_DATE : Color = Color::Rgb(255, 127, 0);
const RAINBOW_YELLOW_FOR_DATE : Color = Color::Rgb(255, 255, 0);
const RAINBOW_GREEN_FOR_DATE : Color = Color::Rgb(0, 255, 0);
const RAINBOW_BLUE_FOR_DATE : Color = Color::Rgb(0, 0, 255);
const RAINBOW_INDIGO_FOR_DATE : Color = Color::Rgb(75, 0, 130);
const RAINBOW_VIOLET_FOR_DATE : Color = Color::Rgb(148, 0, 211);

pub const RAINBOW_STRUCT_FOR_CLOCK : [Color; 7] = [RAINBOW_RED_FOR_CLOCK, RAINBOW_ORANGE_FOR_CLOCK, RAINBOW_YELLOW_FOR_CLOCK , RAINBOW_GREEN_FOR_CLOCK, RAINBOW_BLUE_FOR_CLOCK, RAINBOW_INDIGO_FOR_CLOCK , RAINBOW_VIOLET_FOR_CLOCK];
pub const RAINBOW_STRUCT_FOR_DATE : [Color; 7] = [RAINBOW_RED_FOR_DATE, RAINBOW_ORANGE_FOR_DATE, RAINBOW_YELLOW_FOR_DATE, RAINBOW_GREEN_FOR_DATE, RAINBOW_BLUE_FOR_DATE, RAINBOW_INDIGO_FOR_DATE, RAINBOW_VIOLET_FOR_DATE];

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
#[command(author="Rômulo Moraes", version="0.3.0", about="Digital clock", long_about=None)]
pub struct ProgramArguments{
    #[arg(long, short, help=AVAILABLE_COLORS_FOR_CLOCK)]
    pub clock_color : Option<String>,

    #[arg(long, short='d', help=AVAILABLE_COLORS_FOR_DATE)]
    pub date_color : Option<String>,

    #[arg(long, short='s', help="Make the clock be small")]
    pub small : bool,

    #[arg(long, short='H', help="Set a specific hour to clock")]
    pub hour : Option<u32>,

    #[arg(long, short='m', help="Set a specific minute to clock")]
    pub minute : Option<u32>,

    #[arg(long, short='S', help="Set a specific second to clock")]
    pub second : Option<u32>,

    #[arg(long, short='D', help="Set a specific day to clock")]
    pub day : Option<u32>,

    #[arg(long, short='M', help="Set a specific month to clock")]
    pub month : Option<u32>,

    #[arg(long, short, help="Set a specific year to clock", allow_hyphen_values=true)]
    pub year : Option<i32>

}

// All clock configurations that will be useful at rendering time 
// are stored here
#[derive(Clone)]
pub struct ClockConfig{
    pub clock_color : Option<Color>,
    pub small_clock : bool,
    pub date_color : Option<Color>,
    pub rainbow_mode_for_clock : u8,
    pub rainbow_mode_for_date : bool,
    pub hour : Option<u32>,
    pub minute : Option<u32>,
    pub second : Option<u32>,
    pub day : Option<u32>,
    pub month : Option<u32>,
    pub year : Option<i32>
}