mod calc;

use crate::calc::pi_calculator::{Task, DEFAULT_PRECISION};
use rug::{Float};
use std::env;
use std::thread;
use std::collections::VecDeque;
use std::sync::{mpsc};
use std::sync::mpsc::{Receiver};
use std::time::Instant;


fn handle_args(args: Vec<String>) -> (usize, usize)
{
    let max_thread = num_cpus::get();
    //max_thread = 2;
    let mut result = (0usize, 0usize);
    if args.len() == 5 {
        if args[1] == String::from("-p")
        {
            match args[2].parse::<usize>() {
                Ok(value) => result.0 = value,
                Err(_) => {
                    println!("Invalid argument for -p, setting it to default");
                    result.0 = DEFAULT_PRECISION as usize;
                }
            }
        }
        if args[3] == String::from("-t") || args[3] == String::from("-tasks")
        {
            match args[4].parse::<usize>() {
                Ok(value) => result.1 = value,
                Err(_) => {
                    println!("Invalid argument for -t, setting it to the max threads on the machine");
                    result.1 = max_thread;
                }
            }
        }
    } else if args.len() == 3 {
        if args[1] == String::from("-p")
        {
            match args[2].parse::<usize>() {
                Ok(value) => result.0 = value,
                Err(_) => {
                    println!("Invalid argument for -p, setting it to default");
                    result.0 = DEFAULT_PRECISION as usize;
                }
            }
        }
        result.1 = max_thread;
    } else {
        println!("Invalid command line arguments, setting the default values (-p {} -t {})", DEFAULT_PRECISION, max_thread);
        result.0 = DEFAULT_PRECISION as usize;
        result.1 = max_thread;
    }
    result
}

fn main() {
    //let start_time = Instant::now();
    let (amount_of_elements, thread_count) = handle_args(env::args().collect());
    println!("Starting the program with {} elements to be calculated on {} threads", amount_of_elements, thread_count);

    let mut result = Float::with_val(DEFAULT_PRECISION, 0);
    let mut task_tuples: VecDeque<(u32, u32)> = VecDeque::new();
    let amount_of_work_per_thread = (amount_of_elements / thread_count) / 2;
    //let mut timers: Vec<Instant> = vec!();

    for start_task in (0..amount_of_elements).step_by(amount_of_work_per_thread){
        let end_index: u32 = {
            if start_task + amount_of_work_per_thread < amount_of_elements
            {
                start_task + amount_of_work_per_thread - 1
            }
            else{
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
        //timers.push(Instant::now());
        thread::spawn(move ||{
            println!("Thread-{} has started working", current_thread);
            current_task.work();
        });
        receivers.push(receiver);
    }
    while calculated_elements < (amount_of_elements as u32)
    {
        for current_thread in 0..thread_count{
            match receivers[current_thread].try_recv(){
                Ok(result_from_thread) => {
                    println!("Thread-{} has stoped working", current_thread);
                    //println!("Thread-{}'s time is {:?}", current_thread, timers[current_thread].elapsed());
                    result += result_from_thread.clone();
                    calculated_elements += amount_of_work_per_thread as u32;
                    let (sender, receiver) = mpsc::channel();
                    match task_tuples.pop_front(){
                        Some((start, end)) => {
                            let mut current_task = Task::new(start, end, sender);
                            //timers[current_thread] = Instant::now();
                            thread::spawn(move ||{
                                println!("Thread-{} has started working", current_thread);
                                current_task.work();
                            });
                            receivers[current_thread] = receiver;
                        }
                        None => {
                            break;
                        }
                    };
                },
                Err(_) => continue,
            };
        }
    }
    println!("{}", 1 / result);
    //println!("End time is {:?}", start_time.elapsed());
}
