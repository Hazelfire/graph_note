extern crate ncurses;
use ncurses::*;

use super::database::Database;

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


pub fn display(dbr: Result<Database, Option<String>>)
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
    match dbr {
        Ok(db) => {
            if(db.models.len() == 0){
                addstr("You have no models! Press m to open the model editor, or q to quit");
            }
            else {
                addstr("Welcome back!");
            }
        }
        Err(_) => {
            addstr("Could not load database");
        }
    }

    /* Print some unicode(Chinese) string. */
    // printw("Great Firewall dislike VPN protocol.\nGFW 不喜欢 VPN 协议。");

    /* Update the screen. */
    refresh();

    /* Wait for a key press. */
    let mut ch = getch();
    let mut cursor: Cursor = Cursor{x: 0, y: 0};
    loop {
        cursor.move_from_char(&ch);    
        match std::char::from_u32(ch as u32){
            Some(x) => {
                match x {
                    'q' => {
                        break;
                    },
                    'c' => {
                        
                    }
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
