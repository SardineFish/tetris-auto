use std::{array, collections::BinaryHeap, io::{self, Write, stdin, stdout}, mem, os::macos::fs, process, sync::{Arc, mpsc::channel}, thread};

use termion::{event::Key, input::{TermRead}, raw::IntoRawMode};
use crate::{game::MAX_BRICKS_COUNT, op::GameOPStr};

use crate::{fixed_heap::FixedHeap, game::GameState};

const HEAP_SIZE: usize = 10000;
const EXPAND_SIZE: usize = 34;

pub struct  TetrisAuto {
    
}

impl TetrisAuto{
    pub fn start()  {
        let mut stdin_key = stdin().keys();
        let mut stdout = stdout().into_raw_mode().unwrap();
        let mut curr_heap = FixedHeap::<GameState, HEAP_SIZE>::default();
        let mut next_heap = FixedHeap::<GameState, HEAP_SIZE>::default();

        let (sender, receiver) = channel();

        thread::spawn(move || {
            let stdin = stdin().keys();
            for key in stdin {
                match key {
                    Ok(Key::Ctrl('c')) => sender.send(true).unwrap(),
                    _ => (),
                }
            }
        });

        let mut next_states: [GameState; EXPAND_SIZE] = array_init::array_init(|_| GameState::initial_state());
        let initial_state = GameState::initial_state();
        next_heap.push(initial_state);
        while next_heap.len() > 0 {
            mem::swap(&mut curr_heap, &mut next_heap);
            next_heap.clear();

            curr_heap.peak().unwrap().render();
            if curr_heap.peak().unwrap().brick_count >= MAX_BRICKS_COUNT {
                Self::save_result(curr_heap.peak().unwrap());
                return;
            }
            if let Ok(_) = receiver.try_recv() {
                Self::save_result(curr_heap.peak().unwrap());
                return;
            }
            stdout.flush().unwrap();

            // match stdin_key.next() {
            //     Some(Ok(termion::event::Key::Ctrl('c'))) => process::exit(0),
            //     _ => (),
            // }

            for curr_state in &curr_heap {
                let len = curr_state.next(&mut next_states);
                for next_state in &mut next_states[..len] {
                    let mut temp = GameState::default(); // temp=0, next=full
                    mem::swap(&mut temp, next_state); // temp=full, next=0
                    match next_heap.push(temp) {
                        Some(mut old_state) => mem::swap(next_state, &mut old_state), // old=0, next=full
                        None => *next_state = GameState::initial_state(),
                    }
                }
            }

        }
    }

    pub fn save_result(state: &GameState) {
        let op_str = state.get_op_sequence().to_op_string();
        // stdout.suspend_raw_mode().unwrap();
        std::fs::write("./op_sequence", op_str).unwrap();
    }
}