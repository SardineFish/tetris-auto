use std::{array::IntoIter, mem::{self, MaybeUninit}};

pub struct FixedHeap<T, const N: usize> {
    data: [T; N],
    actual_len: usize,
}

impl<T, const N: usize> Default for FixedHeap<T, N> where T : Default {
    fn default() -> Self {
        let mut array: [T; N] = unsafe { MaybeUninit::uninit().assume_init() };
        for item in &mut array {
            *item = T::default();
        }
        Self {
            data: array,
            actual_len: 0,
        }
    }
}

impl<T, const N: usize> FixedHeap<T, N> where T : Copy + Default {
    fn new() -> Self {
        Self {
            data: [T::default(); N],
            actual_len: 0,
        }
    }
}

impl<T, const N: usize> FixedHeap<T, N> where T : Ord {
    pub fn push(&mut self, element: T) -> Option<T> {
        
        if self.actual_len == self.data.len() {
            Some(self.replace_root(element))
        } else {
            self.push_back(element);
            None
        }
    }
    fn push_back(&mut self, element: T) {
        self.data[self.actual_len] = element;
        let mut pos = self.actual_len;
        self.actual_len += 1;

        while pos > 0 {
            let parent = (pos - 1) / 2;
            if &self.data[parent] > &self.data[pos] {
                self.data.swap(pos, parent);
            }
            pos = parent;
        }
    }

    fn replace_root(&mut self, mut element: T) -> T {
        if self.actual_len == 0 || element < self.data[0] {
            return element;
        }
        mem::swap(&mut self.data[0], &mut element);

        let mut pos = 0;
        while pos < self.actual_len {
            let left_child_idx = pos * 2 + 1;
            let mut right_child_idx = pos * 2 + 2;
            if left_child_idx >= self.actual_len {
                break;
            } else if right_child_idx >= self.actual_len {
                right_child_idx = left_child_idx;
            }
            if self.data[left_child_idx] < self.data[pos] && self.data[left_child_idx] < self.data[right_child_idx] {
                self.data.swap(pos, left_child_idx);
                pos = left_child_idx;
            } else if self.data[left_child_idx] < self.data[pos] || self.data[right_child_idx] <= self.data[pos] {
                self.data.swap(pos, right_child_idx);
                pos = right_child_idx;
            } else {
                break;
            }
         }

        element
    }
}

impl<T, const N: usize> IntoIterator for FixedHeap<T, N> {
    type IntoIter = IntoIter<T, N>;
    type Item = T;
    fn into_iter(self) -> Self::IntoIter {
        IntoIter::new(self.data)
    }
}

#[cfg(test)]
mod test{
    use super::*;
    use rand::Fill;

    #[test]
    fn test(){
        const DATA_SIZE: usize = 32768;
        const HEAP_SIZE: usize = 128;
        type DataType = i32;
        let mut rng = rand::thread_rng();
        let mut data: [DataType; DATA_SIZE] = [DataType::default(); DATA_SIZE];
        data.try_fill(&mut rng).unwrap();

        println!("{:?}", data);
        let mut heap = FixedHeap::<DataType, HEAP_SIZE>::new();
        for x in data {
            heap.push(x);
        }
        let mut heap_data = heap.into_iter().collect::<Vec<_>>();
        heap_data.sort();
        data.sort();

        println!("{:?}", data);
        assert_eq!(&data[(DATA_SIZE - HEAP_SIZE)..], &heap_data[..HEAP_SIZE]);
    }
}