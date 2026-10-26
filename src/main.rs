mod player;
mod map;

use ncurses::{curs_set, endwin, getch, initscr, refresh};

use player::{Player, MoveDir};
use map::Map;

fn main() {
    initscr();
    curs_set(ncurses::CURSOR_VISIBILITY::CURSOR_INVISIBLE);

    let map = Map::new("                                ",
                       "        ------------------      ",
                       "        |                |      ",
                       "        |                |      ",
                       "        |                |      ",
                       "        ---------+--------      ",
                       "                 #              ",
                       "  -------------  #              ",
                       "  |           |  #              ",
                       "  |           +###              ",
                       "  -------------                 ",
                       "                                ",);
    let mut player = Player {
        map: &map,
        pos: (4,14),
    };

    loop {
        map.disp().unwrap();
        player.disp().unwrap();

        match getch() {
            65 => player.mov(MoveDir::Up),
            66 => player.mov(MoveDir::Down),
            68 => player.mov(MoveDir::Left),
            67 => player.mov(MoveDir::Right),
            27 => { endwin(); },
            _ => continue,
        }

        refresh();
    }
}
