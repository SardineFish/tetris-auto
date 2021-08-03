use std::{array, collections::BinaryHeap, io::{Write, stdin, stdout}, mem, process, sync::Arc};

use termion::{input::{TermRead}, raw::IntoRawMode};

use crate::{fixed_heap::FixedHeap, game::GameState, utils::create_array_clone};

const HEAP_SIZE: usize = 12800;

pub struct  TetrisAuto {
    
}

impl TetrisAuto{
    pub fn start()  {
        let mut stdin_key = stdin().keys();
        let mut stdout = stdout().into_raw_mode().unwrap();
        let mut curr_heap = Box::new(FixedHeap::<GameState, HEAP_SIZE>::default());
        let mut next_heap = Box::new(FixedHeap::<GameState, HEAP_SIZE>::default());

        let mut next_states: [GameState; 64] = create_array_clone(GameState::default());
        let initial_state = GameState::initial_state();
        next_heap.push(initial_state);
        while next_heap.len() > 0 {
            mem::swap(&mut curr_heap, &mut next_heap);
            next_heap.clear();

            curr_heap.peak().unwrap().render();
            stdout.flush().unwrap();

            // match stdin_key.next() {
            //     Some(Ok(termion::event::Key::Ctrl('c'))) => process::exit(0),
            //     _ => (),
            // }

            for curr_state in curr_heap.as_ref() {
                let len = curr_state.next(&mut next_states);
                for next_state in &next_states[..len] {
                    next_heap.push(next_state.clone());
                }
            }

        }
    }
}