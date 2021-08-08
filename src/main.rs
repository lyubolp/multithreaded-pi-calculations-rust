mod calc;

use crate::calc::pi_calculator::{Task, DEFAULT_PRECISION};
use rug::{Float};
use std::env;
use std::thread;
use std::collections::VecDeque;
use std::sync::{mpsc};
use std::sync::mpsc::{Receiver};
use std::time::Instant;

use clap::{Arg, App};


fn main() {
    let matches = App::new("multithreaded-pi-calculations")
        .version("1.0.0")
        .arg(Arg::with_name("threads")
            .short("t")
            .long("threads")
            .takes_value(true)
            .help("Amount of threads"))
        .arg(Arg::with_name("elements")
            .short("e")
            .long("elements")
            .takes_value(true)
            .help("Amount of elements"))
        .get_matches();

    static DEFAULT_THREADS: usize = 2;
    static DEFAULT_ELEMENTS: usize = 10000;

    let thread_count: usize = match matches.value_of("threads") {
        Some(value) => {
            value.parse::<usize>().unwrap_or(DEFAULT_THREADS)
        }
        None => DEFAULT_THREADS
    };

    let amount_of_elements: usize = match matches.value_of("elements") {
        Some(value) => {
            value.parse::<usize>().unwrap_or(DEFAULT_ELEMENTS)
        }
        None => DEFAULT_ELEMENTS
    };

    println!("Starting the program with {} elements to be calculated on {} threads", amount_of_elements, thread_count);

    let mut result = Float::with_val(DEFAULT_PRECISION, 0);
    let mut task_tuples: VecDeque<(u32, u32)> = VecDeque::new();
    let amount_divisor: usize = 2;
    let amount_of_work_per_thread = (amount_of_elements / thread_count) / amount_divisor;

    for start_task in (0..amount_of_elements).step_by(amount_of_work_per_thread) {
        let end_index: u32 = {
            if start_task + amount_of_work_per_thread < amount_of_elements
            {
                start_task + amount_of_work_per_thread - 1
            } else {
                amount_of_elements - 1
            }
        } as u32;

        task_tuples.push_back((start_task as u32, end_index))
    }
    let mut receivers: Vec<Receiver<Float>> = vec!();
    let mut calculated_elements: u32 = 0;
    for current_thread in 0..thread_count
    {
        let (sender, receiver) = mpsc::channel();
        let (start, end) = task_tuples.pop_front().unwrap();
        let mut current_task = Task::new(start, end, sender);
        thread::spawn(move || {
            println!("Thread-{} has started working", current_thread);
            current_task.work();
        });
        receivers.push(receiver);
    }
    while calculated_elements < (amount_of_elements as u32)
    {
        for current_thread in 0..thread_count {
            match receivers[current_thread].try_recv() {
                Ok(result_from_thread) => {
                    println!("Thread-{} has stoped working", current_thread);
                    result += result_from_thread.clone();
                    calculated_elements += amount_of_work_per_thread as u32;
                    let (sender, receiver) = mpsc::channel();
                    match task_tuples.pop_front() {
                        Some((start, end)) => {
                            let mut current_task = Task::new(start, end, sender);
                            thread::spawn(move || {
                                println!("Thread-{} has started working", current_thread);
                                current_task.work();
                            });
                            receivers[current_thread] = receiver;
                        }
                        None => {
                            break;
                        }
                    };
                }
                Err(_) => continue,
            };
        }
    }
    let final_result: Float = Float::with_val(DEFAULT_PRECISION, 1) / result;

    println!("Pi is {:.5}", final_result);
}
