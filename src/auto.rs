use crate::{game::MAX_BRICKS_COUNT, game_io::RenderGame, op::GameOPStr};
use bus::{Bus, BusReader};
use rand::prelude::*;
use std::{
    mem,
    sync::{
        mpsc::{channel, Receiver, Sender},
    },
    thread::{self, JoinHandle},
};

use crate::{fixed_heap::FixedHeap, game::GameState, game_io::GameRenderer};

const HEAP_SIZE: usize = 10000;
const EXPAND_SIZE: usize = 34;
const JITTER_RATE: f64 = 0.02;

pub struct TetrisAuto {}

impl TetrisAuto {
    pub fn run_continuous(threads: usize) -> (Bus<()>, JoinHandle<()>) {
        let mut kill_bus = Bus::new(threads);
        let (result_snd, result_rcv) = channel::<GameState>();
        let (resource_sender, resource_receiver) = channel();
        for _ in 0..threads {
            resource_sender.send(kill_bus.add_rx()).unwrap();
        }

        let mut renderer = GameRenderer::new();

        // Result Renderer
        let render_handle = thread::spawn(move || {
            let mut best_state = GameState::default();
            while let Ok(state) = result_rcv.recv() {
                if state.score > best_state.score {
                    best_state = state;
                    renderer.render_game(&best_state);
                    renderer.flush();
                }
                if best_state.brick_count == MAX_BRICKS_COUNT {
                    let seqence = best_state.get_op_sequence().to_op_string();
                    std::fs::write(format!("op_sequence_{}", best_state.score), seqence).unwrap();
                }
            }
        });

        let kill_rx = kill_bus.add_rx();
        thread::spawn(move || {
            Self::spawn_thread(resource_receiver, resource_sender, result_snd, kill_rx);
        });

        (kill_bus, render_handle)
    }
    pub fn spawn_thread(
        res_receiver: Receiver<BusReader<()>>,
        res_sender: Sender<BusReader<()>>,
        result_sender: Sender<GameState>,
        mut kill_rx: BusReader<()>,
    ) {
        while let Ok(mut kill_rcv) = res_receiver.recv() {
            if kill_rx.try_recv().is_ok() {
                return;
            }
            let res_sender = res_sender.clone();
            let result_sender = result_sender.clone();
            thread::spawn(move || {
                let final_state = Self::start(false, &mut kill_rcv, result_sender.clone());
                result_sender.send(final_state).ok();
                res_sender.send(kill_rcv).ok();
            });
        }
    }
    pub fn start(enable_render: bool, kill_signal: &mut BusReader<()>, result_sender: Sender<GameState>) -> GameState {
        // let mut stdin_key = stdin().keys();
        let mut curr_heap = FixedHeap::<GameState, HEAP_SIZE>::default();
        let mut next_heap = FixedHeap::<GameState, HEAP_SIZE>::default();

        let mut renderer = GameRenderer::new();

        let mut rng = rand::thread_rng();

        let mut next_states: [GameState; EXPAND_SIZE] =
            array_init::array_init(|_| GameState::initial_state());
        let initial_state = GameState::initial_state();
        next_heap.push(initial_state);
        while next_heap.len() > 0 {
            mem::swap(&mut curr_heap, &mut next_heap);
            next_heap.clear();

            result_sender.send(curr_heap.peak().unwrap().clone()).unwrap();

            if enable_render {
                renderer.render_game(curr_heap.peak().unwrap());
                renderer.flush();
            }
            if curr_heap.peak().unwrap().brick_count >= MAX_BRICKS_COUNT
                || kill_signal.try_recv().is_ok()
            {
                return curr_heap.peak().unwrap().clone();
            }

            for curr_state in &curr_heap {
                let len = curr_state.next(&mut next_states);
                for next_state in &mut next_states[..len] {
                    next_state.sp_score += (rng.gen_range(-JITTER_RATE..JITTER_RATE)
                        * next_state.sp_score as f64)
                        as i32;
                    let mut temp = GameState::default(); // temp=0, next=full
                    mem::swap(&mut temp, next_state); // temp=full, next=0
                    match next_heap.push(temp) {
                        Some(mut old_state) => mem::swap(next_state, &mut old_state), // old=0, next=full
                        None => *next_state = GameState::initial_state(),
                    }
                }
            }
        }

        curr_heap.peak().unwrap().clone()
    }

    pub fn save_result(state: &GameState) {
        let op_str = state.get_op_sequence().to_op_string();
        // stdout.suspend_raw_mode().unwrap();
        std::fs::write("./op_sequence", op_str).unwrap();
    }
}
