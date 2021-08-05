#[cfg(target_family="unix")]
pub use unix_renderer::{UnixRenderer as GameRenderer, UnixInput as GameInput};
#[cfg(target_family="windows")]
pub use win_renderer::{GameRenderer, WinInput as GameInput};

use crate::{brick::Brick, game::GameState, op::GameOP, vec2::Vec2};

pub trait RenderGame {
    fn render_game(&mut self, state: &GameState);
    fn render_user_hint(&mut self);
    fn render_brick(&mut self, brick: &Brick, pos: Vec2);
    fn flush(&mut self);
}

pub trait GetInput {
    fn get_input(&mut self) -> GameOP;

    fn try_get_interrupt(&mut self) -> Result<(), ()>;
}

#[cfg(target_family="unix")]
mod unix_renderer {

    use std::{io::{Stdin, Stdout, Write, stdin, stdout}, sync::mpsc::{Receiver, channel}, thread};

    use termion::{clear, cursor, event::Key, input::{Keys, TermRead}, raw::{IntoRawMode, RawTerminal}};

    use crate::{game::GameState, op::GameOP, vec2::Vec2};

    use super::{RenderGame, GetInput};
    pub struct UnixRenderer {
        stdout: RawTerminal<Stdout>,
    }

    impl UnixRenderer {
        pub fn new() -> Self {
            Self {
                stdout: stdout().into_raw_mode().unwrap(),
            }
        }
    }

    impl RenderGame for UnixRenderer {
        

        fn render_game(&mut self, state: &GameState) {
            print!("{}", clear::All);
            print!("{}Score: {}", cursor::Goto(13, 3), state.score);
            print!("{}Bricks: {}", cursor::Goto(13, 5), state.brick_count);
            for y in 0..20 {
                for x in 0..10 {
                    match state.grids.get(Vec2(x, y)) {
                        true => print!("{}*", cursor::Goto(x as u16 + 1, y as u16 + 1)),
                        false => print!("{} ", cursor::Goto(x as u16 + 1, y as u16 + 1)),
                    }
                }
            }
            for y in 1..=20 {
                print!("{}|", cursor::Goto(11, y));
            }
            for x in 1..=10 {
                print!("{}-", cursor::Goto(x, 21));
            }
        }

        fn render_user_hint(&mut self) {
            print!("{}<Left|Right|Down>: Move", cursor::Goto(13, 6));
            print!("{}<Up>: Rotate", cursor::Goto(13, 7));
            print!("{}<Space>: Place", cursor::Goto(13, 8));
        }

        fn render_brick(&mut self, brick: &crate::brick::Brick, pos:Vec2) {
            for i in 0..4 {
                let pos = brick.get_pos()[i] + pos + Vec2(1, 1);
                match pos {
                    Vec2(0..=10, 0..=20) => (),
                    _ => continue,
                }
                print!("{}*", cursor::Goto(pos.0 as u16, pos.1 as u16));
            }
        }

        fn flush(&mut self) {
            self.stdout.flush().unwrap();
        }
    }

    pub struct UnixInput {
        key: Keys<Stdin>,
        int_receiver: Receiver<()>,
    }
    
    impl UnixInput {
        pub fn new() -> Self {
            let (sender, receiver) = channel();

            thread::spawn(move || {
                for key in stdin().keys() {
                    match key {
                        Ok(Key::Ctrl('c')) => sender.send(()).unwrap(),
                        _ => continue,
                    }
                }
            });

            Self {
                key: stdin().keys(),
                int_receiver: receiver
            }
        }
    }

    impl GetInput for UnixInput {
        fn get_input(&mut self) -> crate::op::GameOP {
            loop {
                if let Some(key) = self.key.next() {
                    break match key {
                        Ok(Key::Left) => GameOP::Left(1),
                        Ok(Key::Right) => GameOP::Right(1),
                        Ok(Key::Down) => GameOP::Down(1),
                        Ok(Key::Char(' ')) => GameOP::New,
                        Ok(Key::Up) => GameOP::Rotate(1),
                        Err(err) => Err(err).unwrap(),
                        _ => continue,
                    }
                }
            }
            
        }
        fn try_get_interrupt(&mut self) -> Result<(), ()> {
            match self.int_receiver.try_recv() {
                Ok(_) => Ok(()),
                Err(_) => Err(()),
            }
        }
    }
}

#[cfg(target_family="windows")]
mod win_renderer {
    use std::io::{Read, stdin};

    use crate::vec2::Vec2;

    use super::RenderGame;


    pub struct GameRenderer {}

    impl GameRenderer {
        pub fn new() -> Self {
            Self{}
        }
    }

    impl RenderGame for GameRenderer {
        fn render_game(&mut self, state: &crate::game::GameState) {
            for y in 0..20 {
                for x in 0..10 {
                    match state.grids.get(Vec2(x, y)) {
                        true => print!("*"),
                        false => print!(" "),
                    }
                }
            }
            println!("|");
            for x in 1..=10 {
                print!("-");
            }
            println!("");
            println!("Score: {}", state.score);
            println!("Bricks: {}", state.brick_count);
        }

        fn render_user_hint(&mut self) {
            print!("<Left|Right|Down>: Move");
            print!("<Up>: Rotate");
            print!("<Space>: Place");
        }

        fn render_brick(&mut self, brick: &crate::brick::Brick, pos: Vec2) {
            
        }

        fn flush(&mut self) {
            
        }
    }

    pub struct WinInput {

    }

    impl WinInput {
        pub fn new() -> Self {
            Self{}
        }
    }

    impl super::GetInput for WinInput {
        fn get_input(&mut self) -> crate::op::GameOP {
            loop {
                let mut buf = [0u8; 1];
                stdin().read(&mut buf).unwrap();
            }
        }
        fn try_get_interrupt(&mut self) -> Result<(), ()> {
            Err(())
        }
    }
}
