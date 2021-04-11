// use std::{cmp::Ordering, collections::VecDeque, usize};

// struct MonotonicQueue<T>
// where
//     T: Ord + Clone,
// {
//     data: VecDeque<(T, usize)>, // (number: T, num_elements: usize)
//     length: usize,
//     window_length: Option<usize>,
// }

// impl<T> MonotonicQueue<T>
// where
//     T: Ord + Clone,
// {
//     fn new() -> MonotonicQueue<T> {
//         MonotonicQueue {
//             data: VecDeque::new(),
//             length: 0,
//             window_length: None,
//         }
//     }

//     fn with_capacity(len: usize) -> MonotonicQueue<T> {
//         MonotonicQueue {
//             data: VecDeque::with_capacity(len),
//             length: 0,
//             auto_length: None,
//         }
//     }

//     fn push(&mut self, new_ele: T) {
//         let mut num_elements: usize = 0;
//         let mut to_be_truncated_count = 0;

//         for item in self.data.iter().rev() {
//             if item.0.cmp(&new_ele) == Ordering::Less {
//                 num_elements += item.1 + 1;
//                 to_be_truncated_count += 1;
//             } else {
//                 break;
//             }
//         }

//         self.data.truncate(self.data.len() - to_be_truncated_count);
//         self.data.push_back((new_ele, num_elements));
//         self.length += 1;

//         match self.auto_length {
//             Some(x) if self.length > x => {
//                 self.pop();
//             }
//             _ => {}
//         }
//     }

//     fn pop(&mut self) {
//         match self.data.front_mut() {
//             Some(x) => {
//                 if x.1 > 0 {
//                     x.1 -= 1;
//                 } else {
//                     self.data.pop_front();
//                 }
//             }
//             _ => {}
//         }

//         self.length -= 1;
//     }
// }

fn main() {
    println!("{}", i16::MAX);
//    let mut q = MonotonicQueue::new();

//    let v1: Vec<i32> = vec![1, 3, 3, -1, -3, 5, 3, 6, 7];
//    let window_size = 4;

//    q.set_auto_len(window_size);

//    for item in v1.iter() {
//        println!("before push data: {:?}", q.data);
//        println!("before push length: {:?}", q.length);
//        println!("push {}", *item);
//        q.push(*item);
//        println!("after push data: {:?}", q.data);
//        println!("after push length: {:?}\n", q.length);
//    }
}
