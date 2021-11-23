use num_bigint::BigUint;
use num_traits::{Zero, One};
use std::mem::replace;
use std::{ thread, sync::{ Arc, Mutex }};
use std::time::{Duration, Instant};

use std::io::{self, BufRead};
use std::sync::mpsc::{self, TryRecvError};

extern crate num_cpus;

// def fib_gen():
//     a, b = 0, 1
//     while True:
//         yield a
//         a, b = b, a + b

// n = 1000000

// f = fib_gen()
// for _ in range(n):
//     next(f)

///
/// Source from:
/// https://vladris.com/blog/2018/02/11/fibonacci.html
/// 
/// 
fn mul2x2(a: &[[BigUint; 2]; 2], b: &[[BigUint; 2]; 2]) -> [[BigUint; 2]; 2] {
  [
      [&a[0][0] * &b[0][0] + &a[1][0] * &b[0][1], &a[0][0] * &b[1][0] + &a[1][0] * &b[1][1]],
      [&a[0][1] * &b[0][0] + &a[1][1] * &b[0][1], &a[0][1] * &b[1][0] + &a[1][1] * &b[1][1]],
  ]
}

fn fast_exp2x2(a: [[BigUint; 2]; 2], n: i32) -> [[BigUint; 2]; 2] {
  op_n_times(a, &mul2x2, n)
}

fn op_n_times<T, Op>(a: T, op: &Op, n: i32) -> T
    where Op: Fn(&T, &T) -> T {
    if n == 1 { return a; }

    let mut result = op_n_times::<T, Op>(op(&a, &a), &op, n >> 1);
    if n & 1 == 1 {
        result = op(&a, &result);
    }

    result
}

fn fib4(n: i32) -> BigUint {
  if n == 0 { return Zero::zero(); }
  if n == 1 { return One::one(); }

  let a = [
      [One::one(), One::one()],
      [One::one(), Zero::zero()],
  ];

  fast_exp2x2(a, n - 1)[0][0].clone()
}

fn main() {

  let n = 3000000;

  let start = Instant::now();

  // for _ in 0..n {
  //     let c = a + &b;
  //     // This is a low cost way of swapping f0 with f1 and f1 with f2.
  //     a = replace(&mut b, c);
  // }
  let result1 = fib4(n/3);
  let result2 = fib4(n/2);
  let result3 = fib4(n);

  let duration = start.elapsed();

  println!("Time elapsed: {:?}", duration);

}

// fn main() {

//   let n = 3000000;
//   let mut a: BigUint = Zero::zero();
//   let mut b: BigUint = One::one();
//   let mut c: BigUint;

//   let start = Instant::now();

//   for _ in 0..n {
//       let c = a + &b;
//       // This is a low cost way of swapping f0 with f1 and f1 with f2.
//       a = replace(&mut b, c);
//   }

//   let duration = start.elapsed();

//   print!("a: {}, b: {}", a, b);

//   println!("Time elapsed: {:?}", duration);

// }

// fn main() {

//   let cores = num_cpus::get() * 100;

//   println!("Starting process using {} cores", cores);
//   thread::sleep(Duration::from_secs(2));

//   let first_term: Arc<Mutex<BigUint>> = Arc::new(Mutex::new(Zero::zero()));
//   let last_term: Arc<Mutex<BigUint>> = Arc::new(Mutex::new(One::one()));

//   println!("Hehe {}", 1);
  
//   let mut handles: Vec<thread::JoinHandle<()>> = Vec::new();

//   for i in 0..cores {
//     let first_term: Arc<Mutex<BigUint>> = Arc::clone(&first_term);
//     let last_term: Arc<Mutex<BigUint>> = Arc::clone(&last_term);
//     let mut count = 0;
//     let handle: thread::JoinHandle<()> = thread::spawn(move || {
//       loop {
//         let temp: BigUint = last_term.lock().unwrap().clone();
//         let sum: BigUint = first_term.lock().unwrap().clone() + temp.clone();

//         let mut first_term_mut = first_term.lock().unwrap();
//         // *first_term_mut = temp;
//         replace(&mut *first_term_mut, temp);

//         let mut last_term_mut = last_term.lock().unwrap();
//         replace(&mut *last_term_mut, sum);
//         // println!("\n\n{}", last_term_mut);

//         count += 1;
//         if count % 1000 == 0 {
//           count = 0;
//           println!("{}\n", last_term_mut);
//         }
//       }
//     });
//     handles.push(handle);
//   }

//   for thread in handles.into_iter() {
//     thread.join().unwrap();
//   }
// }