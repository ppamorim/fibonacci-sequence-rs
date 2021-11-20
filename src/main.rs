use num_bigint::BigUint;
use num_traits::{Zero, One};
use std::time::Duration;
use std::mem::replace;
use std::{ thread, sync::{ Arc, Mutex }};

use std::io::{self, BufRead};
use std::sync::mpsc::{self, TryRecvError};

extern crate num_cpus;

fn main() {

  let cores = num_cpus::get() * 100;

  println!("Starting process using {} cores", cores);
  thread::sleep(Duration::from_secs(2));

  let first_term: Arc<Mutex<BigUint>> = Arc::new(Mutex::new(Zero::zero()));
  let last_term: Arc<Mutex<BigUint>> = Arc::new(Mutex::new(One::one()));

  println!("Hehe {}", 1);
  
  let mut handles: Vec<thread::JoinHandle<()>> = Vec::new();

  for i in 0..cores {
    let first_term: Arc<Mutex<BigUint>> = Arc::clone(&first_term);
    let last_term: Arc<Mutex<BigUint>> = Arc::clone(&last_term);
    let mut count = 0;
    let handle: thread::JoinHandle<()> = thread::spawn(move || {
      loop {
        let temp: BigUint = last_term.lock().unwrap().clone();
        let sum: BigUint = first_term.lock().unwrap().clone() + temp.clone();

        let mut first_term_mut = first_term.lock().unwrap();
        // *first_term_mut = temp;
        replace(&mut *first_term_mut, temp);

        let mut last_term_mut = last_term.lock().unwrap();
        replace(&mut *last_term_mut, sum);
        // println!("\n\n{}", last_term_mut);

        count += 1;
        if count % 1000 == 0 {
          count = 0;
          println!("{}\n", last_term_mut);
        }
      }
    });
    handles.push(handle);
  }

  for thread in handles.into_iter() {
    thread.join().unwrap();
  }
}