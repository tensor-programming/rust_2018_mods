// Rust 2018 vs 2015

// extern crate rand;

// #[macro_use]
// extern crate lazy_static;

// use lazy_static::lazy_static;
// use rand;

// fn main() {
//     lazy_static! {};
//     rand::random::<u8>();
// }

// extern crate rand as r;

// use rand as r;

// fn main() {
//     let x = r::random::<u8>();
// }

// mod sub {
//     fn example() {
//         let x = ::rand::random::<u8>();
//         let x = rand::random::<u8>();
//     }
// }

// mod example;

// mod sub {
//     mod test {
//         use crate::example::sub;
//         use crate::example::sub2;

//         pub(in crate::sub) fn a_func() {
//             sub::sub();
//             sub2::sub();
//         }

//         pub(crate) fn a_func() {
//             sub::sub();
//             sub2::sub();
//         }
//     }
// }

// mod test {
//     use rand::random;

//     mod sub {
//         pub(in crate::test) struct Sub;
//     }

//     use sub::Sub;

//     pub enum AEnum {
//         V1(u8),
//         V2(String),
//     }

//     fn a_func() {
//         let x = std::sync::Arc::new(10);

//         use AEnum::*;
//         let a = V1(10);

//         match a {
//             V1(i) => println!("{}", i),
//             V2(s) => println!("{}", s),
//         }
//     }
// }

// use std::fs::copy;
// use std::io::BufRead;
// use std::path::{Path, PathBuf};

// use std::{
//     fs::copy,
//     io::BufRead,
//     path::{Path, PathBuf},
// };

// fn main() {}
