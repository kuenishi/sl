// extern crate getopts;

extern crate ncurses;

use std::{thread, time};

pub const D51HIGHT : u32 = 10;
pub const D51FUNNEL : u32 = 7;
pub const D51LENGTH : u32 = 54;
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

fn print_line_left(y : i32, x : i32, width : usize, line : &'static str) {
    let (s, _) = line.split_at(width);
    ncurses::mvaddch(y, x, ' ' as u64);
    ncurses::printw(s);
    ncurses::printw(" ");
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

fn print_line_right(y : i32, width : usize, line : &'static str) {
    let (_, s) = line.split_at(width);
    ncurses::mvaddch(y, 0, ' ' as u64);
    ncurses::printw(s);
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
        print_line_left(center - 5, x as i32, width as usize, D51STR1);
        print_line_left(center - 4, x as i32, width as usize, D51STR2);
        print_line_left(center - 3, x as i32, width as usize, D51STR3);
        print_line_left(center - 2, x as i32, width as usize, D51STR4);
        print_line_left(center - 1, x as i32, width as usize, D51STR5);
        print_line_left(center, x as i32, width as usize, D51STR6);
        print_line_left(center + 1, x as i32, width as usize, D51STR7);
        print_line_left(center + 2, x as i32, width as usize, choose_wheel(x, 0));
        print_line_left(center + 3, x as i32, width as usize, choose_wheel(x, 1));
        print_line_left(center + 4, x as i32, width as usize, choose_wheel(x, 2));
        ncurses::refresh();
        thread::sleep(time::Duration::from_millis(40));
    }
    for x in 0..D51LENGTH {
        let width = x;
        print_line_right(center - 5, width as usize, D51STR1);
        print_line_right(center - 4, width as usize, D51STR2);
        print_line_right(center - 3, width as usize, D51STR3);
        print_line_right(center - 2, width as usize, D51STR4);
        print_line_right(center - 1, width as usize, D51STR5);
        print_line_right(center + 0, width as usize, D51STR6);
        print_line_right(center + 1, width as usize, D51STR7);
        let w = (D51LENGTH - x) as u32;
        print_line_right(center + 2, width as usize, choose_wheel(w, 0));
        print_line_right(center + 3, width as usize, choose_wheel(w, 1));
        print_line_right(center + 4, width as usize, choose_wheel(w, 2));
        ncurses::refresh();
        thread::sleep(time::Duration::from_millis(40));
    }
}

pub fn teardown() {
    //    ncurses::getch();
    ncurses::endwin();
}
