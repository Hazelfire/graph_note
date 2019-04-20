/*
   Copyright © 2013 Free Software Foundation, Inc
   See licensing in LICENSE file

   File: examples/ex_1.rs
   Author: Jesse 'Jeaye' Wilkerson
   Description:
   Simple "Hello, world" example.
   */

extern crate ncurses;

use ncurses::*;

struct Cursor {
    x: i32,
    y: i32,
}

trait CursorControl {
    fn move_from_char(&mut self, ch: &i32);
}

impl CursorControl for Cursor {
    fn move_from_char(&mut self, ch: &i32){
        match *ch {
            KEY_RIGHT => {
                self.x += 1;
            },
            KEY_LEFT => {
                if self.x > 0 {
                    self.x -= 1;
                }
            },
            KEY_UP => {
                if self.y > 0 {
                    self.y -= 1;
                }
            },
            KEY_DOWN => {
                self.y += 1;
            },
            _ => {}
        }
        
    }
}

fn keyboard_down(i32 key){

}

fn main()
{
    /* If your locale env is unicode, you should use `setlocale`. */
    // let locale_conf = LcCategory::all;
    // setlocale(locale_conf, "zh_CN.UTF-8"); // if your locale is like mine(zh_CN.UTF-8).

    /* Start ncurses. */
    initscr();
    cbreak();
    keypad(stdscr(), true);
    noecho();

    /* Print to the back buffer. */
    printw("name: Sam
playerName: Kartelei
scores:
  dex: 3
  con: 2
  int: 5
ac: 6");

    /* Print some unicode(Chinese) string. */
    // printw("Great Firewall dislike VPN protocol.\nGFW 不喜欢 VPN 协议。");

    /* Update the screen. */
    refresh();

    /* Wait for a key press. */
    let mut ch = getch();
    let mut cursor: Cursor = Cursor{x: 0, y: 0};
    loop {
        cursor.move_from_char(&ch);    
        keyboard_down(ch);
        match std::char::from_u32(ch as u32){
            Some(x) => {
                match x {
                    'q' => {
                        break;
                    },
                    _ => {}
                }        
            },
            None => {}
        }
        mv(cursor.y, cursor.x);
        refresh();
        ch = getch();
    }

    /* Terminate ncurses. */
    endwin();
}
