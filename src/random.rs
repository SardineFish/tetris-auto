
const RANDOM_A: i32 = 27073; // 乘子
const RANDOM_M: i32 = 32749; // 模数
const RANDOM_C: i32 = 17713; // 增量
pub const RANDOM_SEED: i32 = 12358; // 随机数种子

pub fn get_random_num(v: i32) -> i32 {
    (v * RANDOM_A + RANDOM_C) % RANDOM_M
}