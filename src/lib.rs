// extern crate getopts;

extern crate ncurses;

use std::{thread, time};

pub const D51HIGHT : u32 = 10;
pub const D51FUNNEL : u32 = 7;
pub const D51LENGTH : u32 = 84;
pub const D51PATTERNS : u32 = 6;

pub const D51STR1 : &'static str = "      ====        ________                ___________ ";
pub const D51STR2 : &'static str = "  _D _|  |_______/        \\__I_I_____===__|_________| ";
pub const D51STR3 : &'static str = "   |(_)---  |   H\\________/ |   |        =|___ ___|   ";
pub const D51STR4 : &'static str = "   /     |  |   H  |  |     |   |         ||_| |_||   ";
pub const D51STR5 : &'static str = "  |      |  |   H  |__--------------------| [___] |   ";
pub const D51STR6 : &'static str = "  | ________|___H__/__|_____/[][]~\\_______|       |   ";
pub const D51STR7 : &'static str = "  |/ |   |-----------I_____I [][] []  D   |=======|__ ";

pub const D51WHL11 : &'static str = "__/ =| o |=-~~\\  /~~\\  /~~\\  /~~\\ ____Y___________|__ ";
pub const D51WHL12 : &'static str = " |/-=|___|=    ||    ||    ||    |_____/~\\___/        ";
pub const D51WHL13 : &'static str = "  \\_/      \\O=====O=====O=====O_/      \\_/            ";

pub const D51WHL21 : &'static str = "__/ =| o |=-~~\\  /~~\\  /~~\\  /~~\\ ____Y___________|__ ";
pub const D51WHL22 : &'static str = " |/-=|___|=O=====O=====O=====O   |_____/~\\___/        ";
pub const D51WHL23 : &'static str = "  \\_/      \\__/  \\__/  \\__/  \\__/      \\_/            ";

pub const D51WHL31 : &'static str = "__/ =| o |=-O=====O=====O=====O \\ ____Y___________|__ ";
pub const D51WHL32 : &'static str = " |/-=|___|=    ||    ||    ||    |_____/~\\___/        ";
pub const D51WHL33 : &'static str = "  \\_/      \\__/  \\__/  \\__/  \\__/      \\_/            ";

pub const D51WHL41 : &'static str = "__/ =| o |=-~O=====O=====O=====O\\ ____Y___________|__ ";
pub const D51WHL42 : &'static str = " |/-=|___|=    ||    ||    ||    |_____/~\\___/        ";
pub const D51WHL43 : &'static str = "  \\_/      \\__/  \\__/  \\__/  \\__/      \\_/            ";

pub const D51WHL51 : &'static str = "__/ =| o |=-~~\\  /~~\\  /~~\\  /~~\\ ____Y___________|__ ";
pub const D51WHL52 : &'static str = " |/-=|___|=   O=====O=====O=====O|_____/~\\___/        ";
pub const D51WHL53 : &'static str = "  \\_/      \\__/  \\__/  \\__/  \\__/      \\_/            ";

pub const D51WHL61 : &'static str = "__/ =| o |=-~~\\  /~~\\  /~~\\  /~~\\ ____Y___________|__ ";
pub const D51WHL62 : &'static str = " |/-=|___|=    ||    ||    ||    |_____/~\\___/        ";
pub const D51WHL63 : &'static str = "  \\_/      \\_O=====O=====O=====O/      \\_/            ";

pub const D51DEL  : &'static str = "                                                      ";

pub const  COAL01 : &'static str = "                              ";
pub const  COAL02 : &'static str = "                              ";
pub const  COAL03 : &'static str = "    _________________         ";
pub const  COAL04 : &'static str = "   _|                \\_____A  ";
pub const  COAL05 : &'static str = " =|                        |  ";
pub const  COAL06 : &'static str = " -|                        |  ";
pub const  COAL07 : &'static str = "__|________________________|_ ";
pub const  COAL08 : &'static str = "|__________________________|_ ";
pub const  COAL09 : &'static str = "   |_D__D__D_|  |_D__D__D_|   ";
pub const  COAL10 : &'static str = "    \\_/   \\_/    \\_/   \\_/    ";

pub const  COALDEL : &'static str = "                              ";

fn smokes(y : i32, x : i32) {
    smoke(y, x, 0);
    smoke(y - 2, x + 2, 1);
    smoke(y - 3, x + 4, 2);
    smoke(y - 4, x + 6, 3);
    for i in 4..15 {
        smoke(y - 5, x + 4 + i, i as usize)
    }
}

fn smoke(y : i32, x : i32, n : usize) {
    let smokes : [[&'static str; 16]; 2] =
        [["(   )", "(    )", "(    )", "(   )", "(  )",
         "(  )" , "( )"   , "( )"   , "()"   , "()"  ,
         "O"    , "O"     , "O"     , "O"    , "O"   ,
          " "],
         ["(@@@)", "(@@@@)", "(@@@@)", "(@@@)", "(@@)",
          "(@@)" , "(@)"   , "(@)"   , "@@"   , "@@"  ,
          "@"    , "@"     , "@"     , "@"    , "@"   ,
          " "]];
    let eraser : [&'static str; 16] =
        ["     ", "      ", "      ", "     ", "    ",
         "    " , "   "   , "   "   , "  "   , "  "  ,
         " "    , " "     , " "     , " "    , " "   ,
         " " ];
    let a = mymod(n as u32, 16) as usize;
    if mymod(x as u32, 4) == 0 {
        if n < 4 {
            ncurses::mvaddstr(y, x+4, eraser[a]);
        }
        let b = mymod((x / 8) as u32, 2);
        ncurses::mvaddstr(y, x, smokes[b as usize][a]);
        //3 => ncurses::printw(eraser[a]),
    };
}

fn print_line_left(y : i32, x : i32, width : usize, line : &str) {
    let (s, _) = line.split_at(width);
    ncurses::mvaddstr(y, x, s);
}

// x % y
fn mymod(x : u32, y : u32) -> u32 {
    let mut a = x;
    while a >= y {
        a -= y;
    }
    a
}

fn choose_wheel(x : u32, h : u32) -> &'static str {
    match (mymod(x as u32, D51PATTERNS), h) {
        (0, 0) => D51WHL11,
        (0, 1) => D51WHL12,
        (0, 2) => D51WHL13,
        (1, 0) => D51WHL21,
        (1, 1) => D51WHL22,
        (1, 2) => D51WHL23,
        (2, 0) => D51WHL31,
        (2, 1) => D51WHL32,
        (2, 2) => D51WHL33,
        (3, 0) => D51WHL41,
        (3, 1) => D51WHL42,
        (3, 2) => D51WHL43,
        (4, 0) => D51WHL51,
        (4, 1) => D51WHL52,
        (4, 2) => D51WHL53,
        (5, 0) => D51WHL61,
        (5, 1) => D51WHL62,
        (5, 2) => D51WHL63,
        _ => D51DEL
    }
}

fn train_line(x : u32, h : u32) -> String {
    let (former, latter) = match h {
        0 => (D51STR1, COAL01),
        1 => (D51STR2, COAL02),
        2 => (D51STR3, COAL03),
        3 => (D51STR4, COAL04),
        4 => (D51STR5, COAL05),
        5 => (D51STR6, COAL06),
        6 => (D51STR7, COAL07),
        7 => (choose_wheel(x, 0), COAL08),
        8 => (choose_wheel(x, 1), COAL09),
        9 => (choose_wheel(x, 2), COAL10),
        _ => (D51DEL, COALDEL)
    };
    former.to_string() + latter
}

fn print_line_right(y : i32, width : usize, line : &str) {
    let (_, s) = line.split_at(width);
    ncurses::mvaddstr(y, 0, s);
    for i in (line.len() - width)..(line.len()) {
        ncurses::mvaddch(y, i as i32, ' ' as u64);
    }
}

pub fn setup() {
    ncurses::initscr();
    ncurses::raw();
    ncurses::noecho();
    ncurses::curs_set(ncurses::CURSOR_VISIBILITY::CURSOR_INVISIBLE);
    ncurses::nodelay(ncurses::stdscr, true);
    ncurses::leaveok(ncurses::stdscr, true);
    ncurses::scrollok(ncurses::stdscr, false);
}

pub fn run() {
    let mut max_x = 0;
    let mut max_y = 0;
    ncurses::getmaxyx(ncurses::stdscr, &mut max_y, &mut max_x);
    ncurses::refresh();

    let center = max_y / 2;

    for i in 1..max_x {
        let width = std::cmp::min(D51LENGTH, i as u32) - 1;
        let x : u32 = (max_x - i) as u32;
        smokes(center - 7, x as i32 + 7);
        print_line_left(center - 5, x as i32, width as usize, &train_line(x, 0));
        print_line_left(center - 4, x as i32, width as usize, &train_line(x, 1));
        print_line_left(center - 3, x as i32, width as usize, &train_line(x, 2));
        print_line_left(center - 2, x as i32, width as usize, &train_line(x, 3));
        print_line_left(center - 1, x as i32, width as usize, &train_line(x, 4));
        print_line_left(center, x as i32, width as usize, &train_line(x, 5));
        print_line_left(center + 1, x as i32, width as usize, &train_line(x, 6));
        print_line_left(center + 2, x as i32, width as usize, &train_line(x, 7));
        print_line_left(center + 3, x as i32, width as usize, &train_line(x, 8));
        print_line_left(center + 4, x as i32, width as usize, &train_line(x, 9));
        ncurses::refresh();
        thread::sleep(time::Duration::from_millis(40));
    }
    for x in 0..D51LENGTH {
        let width = x;
        print_line_right(center - 5, width as usize, &train_line(x, 0));
        print_line_right(center - 4, width as usize, &train_line(x, 1));
        print_line_right(center - 3, width as usize, &train_line(x, 2));
        print_line_right(center - 2, width as usize, &train_line(x, 3));
        print_line_right(center - 1, width as usize, &train_line(x, 4));
        print_line_right(center + 0, width as usize, &train_line(x, 5));
        print_line_right(center + 1, width as usize, &train_line(x, 6));
        print_line_right(center + 2, width as usize, &train_line(x, 7));
        print_line_right(center + 3, width as usize, &train_line(x, 8));
        print_line_right(center + 4, width as usize, &train_line(x, 9));
        ncurses::refresh();
        thread::sleep(time::Duration::from_millis(40));
    }
}

pub fn teardown() {
    //    ncurses::getch();
    ncurses::endwin();
}
