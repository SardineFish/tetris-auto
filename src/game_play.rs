use std::{io::{self, Stdout, Write, stdout}, process, sync::mpsc::channel, thread};

use crate::{brick::Brick, game::{self, GameState}, op::GameOP, vec2::{self, Vec2}};
use termion::{event::Key, clear, cursor, input::TermRead, raw::{IntoRawMode, RawTerminal}};

pub struct Game {
    state: GameState,
    brick_pos: Vec2,
    brick: Brick,
    stdout: RawTerminal<Stdout>
}

impl Game {
    pub fn new() -> Self {
        Self {
            state: GameState::initial_state(),
            brick: Brick::from_random_num(0, 0),
            brick_pos: game::INITIAL_POS,
            stdout: io::stdout().into_raw_mode().unwrap(),
        }
    }

    pub fn start(mut self) {
        let (sender, receiver) = channel();
        let mut stdout = io::stdout().into_raw_mode().unwrap();
        write!(stdout, "{}", clear::All);
        let join = thread::spawn(move || {
            let mut stdin = io::stdin();
            for key in stdin.keys() {
                let op = match key {
                    Ok(Key::Left) => GameOP::Left(1),
                    Ok(Key::Right) => GameOP::Right(1),
                    Ok(Key::Down) => GameOP::Down(1),
                    Ok(Key::Char(' ')) => GameOP::New,
                    Ok(Key::Up) => GameOP::Rotate(1),
                    Ok(Key::Ctrl('c')) => {
                        write!(stdout, "{}", cursor::Show);
                        stdout.suspend_raw_mode();
                        drop(stdout);
                        process::exit(0);
                    },
                    _ => continue,
                };
                sender.send(op).unwrap();
            }
        });

        self.render();
        
        loop {
            let op = receiver.recv().unwrap();
            if !self.update(op) {
                self.render();
                println!("Game Over!");
                return;
            }
            
            self.render();
        }
    }

    pub fn update(&mut self, op: GameOP) -> bool
    {
        
        let next_pos = match op {
            GameOP::New => {
                game::INITIAL_POS
            },
            GameOP::Rotate(rot) => {
                self.brick_pos
            },
            GameOP::Left(dx) => self.brick_pos + Vec2(-dx, 0),
            GameOP::Right(dx) => self.brick_pos + Vec2(dx, 0),
            GameOP::Down(dy) => self.brick_pos + Vec2(0, dy),
        };

        if self.state.grids.brick_pos_valid(&self.brick, next_pos, true) {
            match op {
                GameOP::New => {
                    self.state.grids.place_teris_brick(&self.brick, self.brick_pos);
                    self.state.evaluate_score();
                    let next_brick = self.state.next_brick();
                    if !self.state.grids.brick_pos_valid(&next_brick, game::INITIAL_POS, true) {
                        return false;
                    }
                    self.brick = next_brick;
                },
                GameOP::Rotate(rot) => {
                    let mut brick = self.brick;
                    for _ in 0..rot {
                        brick = brick.rotate();
                    }
                    if !self.state.grids.brick_pos_valid(&brick, self.brick_pos, true) {
                        return true;
                    }
                    self.brick = brick;
                }
                _ =>(),
            }
            self.brick_pos = next_pos;
        }

        true
    }

    pub fn render(&mut self) {
        
        print!("{}", clear::All);
        print!("{}Score:\n {}", cursor::Goto(13, 3), self.state.score);
        print!("{}<Left|Right|Down>: Move", cursor::Goto(13, 6));
        print!("{}<Up>: Rotate", cursor::Goto(13, 7));
        print!("{}<Space>: Place", cursor::Goto(13, 8));
        for y in 0..20 {
            for x in 0..10 {
                match self.state.grids.get(Vec2(x, y)) {
                    true => print!("{}*", cursor::Goto(x as u16 + 1, y as u16 + 1)),
                    false => print!("{} ", cursor::Goto(x as u16 + 1, y as u16 + 1)),
                }
            }
        }
        for i in 0..4 {
            let pos = self.brick.get_pos()[i] + self.brick_pos + Vec2(1, 1);
            match pos {
                Vec2(0..=10, 0..=20) => (),
                _ => continue,
            }
            print!("{}*", cursor::Goto(pos.0 as u16, pos.1 as u16));
        }
        for y in 1..=20 {
            print!("{}|", cursor::Goto(11, y));
        }
        for x in 1..=10 {
            print!("{}-", cursor::Goto(x, 21));
        }
        
        
        println!("{}", cursor::Hide);
        self.stdout.flush().unwrap();
    }
}