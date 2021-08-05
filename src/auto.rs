use std::mem;
use rand::prelude::*;
use crate::{game::MAX_BRICKS_COUNT, game_io::{GetInput, RenderGame}, op::GameOPStr};

use crate::{fixed_heap::FixedHeap, game::GameState, game_io::{GameInput, GameRenderer}};

const HEAP_SIZE: usize = 10000;
const EXPAND_SIZE: usize = 34;
const JITTER_RATE: f64 = 0.02;

pub struct  TetrisAuto {
    
}

impl TetrisAuto{
    pub fn start()  {
        // let mut stdin_key = stdin().keys();
        let mut curr_heap = FixedHeap::<GameState, HEAP_SIZE>::default();
        let mut next_heap = FixedHeap::<GameState, HEAP_SIZE>::default();

        let mut input = GameInput::new();
        let mut renderer = GameRenderer::new();

        let mut rng = rand::thread_rng();

        let mut next_states: [GameState; EXPAND_SIZE] = array_init::array_init(|_| GameState::initial_state());
        let initial_state = GameState::initial_state();
        next_heap.push(initial_state);
        while next_heap.len() > 0 {
            mem::swap(&mut curr_heap, &mut next_heap);
            next_heap.clear();

            renderer.render_game(curr_heap.peak().unwrap());
            if curr_heap.peak().unwrap().brick_count >= MAX_BRICKS_COUNT {
                Self::save_result(curr_heap.peak().unwrap());
                return;
            }
            if let Ok(_) = input.try_get_interrupt() {
                Self::save_result(curr_heap.peak().unwrap());
                return;
            }
            renderer.flush();

            // match stdin_key.next() {
            //     Some(Ok(termion::event::Key::Ctrl('c'))) => process::exit(0),
            //     _ => (),
            // }

            for curr_state in &curr_heap {
                let len = curr_state.next(&mut next_states);
                for next_state in &mut next_states[..len] {
                    next_state.sp_score += (rng.gen_range(-JITTER_RATE..JITTER_RATE) * next_state.sp_score as f64) as i32;
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