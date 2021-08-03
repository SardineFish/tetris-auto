use std::mem::MaybeUninit;

pub fn create_array_clone<T: Clone, const N: usize>(val: T) -> [T; N] {
    let mut array: [T; N] = unsafe { MaybeUninit::uninit().assume_init() };
    for item in &mut array {
        *item = val.clone();
    }
    array
}