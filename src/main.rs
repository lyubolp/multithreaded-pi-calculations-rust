mod calc;

use crate::calc::pi_calculator::{Task, DEFAULT_PRECISION};
use rug::{Float};
use std::env;
use std::thread;
use std::thread::JoinHandle;
use std::collections::VecDeque;
use std::sync::{mpsc};
use std::ptr::null;
use std::sync::mpsc::{Sender, Receiver};


fn handle_args(args: Vec<String>) -> (usize, usize)
{
    let mut max_thread = num_cpus::get();
    max_thread = 2;
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
    let (amount_of_elements, thread_count) = handle_args(env::args().collect());
    println!("Starting the program with {} elements to be calculated on {} threads", amount_of_elements, thread_count);

    let mut result = Float::with_val(DEFAULT_PRECISION, 0);


    let mut task_tuples: VecDeque<(u32, u32)> = VecDeque::new();
    let amount_of_work_per_thread = (amount_of_elements / thread_count) / 2;

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
        thread::spawn(move ||{
            current_task.work();
        });
        receivers.push(receiver);
    }
    while calculated_elements < (amount_of_elements as u32)
    {
        for current_thread in 0..thread_count{
            match receivers[current_thread].try_recv(){
                Ok(result_from_thread) => {
                    result += result_from_thread.clone();
                    calculated_elements += amount_of_work_per_thread as u32;
                    let (sender, receiver) = mpsc::channel();
                    match task_tuples.pop_front(){
                        Some((start, end)) => {
                            let mut current_task = Task::new(start, end, sender);
                            thread::spawn(move ||{
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


    /*let (sender, receiver) = mpsc::channel();

    let mut current_task = Task::new(0, 100, sender);
    let handle = thread::spawn(move ||{
        current_task.work();
    });

    handle.join();
    match receiver.try_recv(){
        Ok(result_from_thread)=> result = result_from_thread,
        Err(_) => {}
    }*/



    // let mut elements: Vec<Float> = vec![Float::new(DEFAULT_PRECISION); amount_of_elements as usize];
    // let mut tasks_for_thread: Vec<VecDeque<u32>> = vec![VecDeque::new(); thread_count];
    // let mut all_tasks: VecDeque<u32> = VecDeque::new();
    //
    // for i in 0..amount_of_elements {
    //     all_tasks.push_back(i as u32);
    // }
    //
    // let amount_of_work_per_thread = ((amount_of_elements as f32).ln() / (thread_count as f32).ln()) as usize;
    // let mut last_calculated_index: Vec<u32> = vec![0; thread_count];
    // let mut calculated_elements: u32 = 0;
    //
    // for current_thread in 0..thread_count {
    //     for i in 0..amount_of_work_per_thread {
    //         tasks_for_thread[current_thread as usize].push_back(all_tasks.pop_front().unwrap());
    //     }
    // }
    //
    // for current_thread in 0..thread_count {
    //     elements[(amount_of_work_per_thread * current_thread) as usize] =
    //         calculate_a_n_from_formula(tasks_for_thread[current_thread].pop_front() as u32, DEFAULT_PRECISION);
    //
    //     last_calculated_index[current_thread] = (amount_of_work_per_thread * current_thread) as u32;
    //     calculated_elements += 1;
    //     result += elements[(amount_of_work_per_thread * current_thread) as usize].clone();
    // }
    //
    //
    // let mut threads_handles: Vec<JoinHandle<()>> = vec!();
    // let mut channels: Vec<mpsc::Receiver<(Float, u32)>> = vec!();
    //
    // for current_thread in 0..thread_count {
    //     let current_task = Task::new(
    //         (elements[(current_thread * amount_of_work_per_thread) as usize]).clone(),
    //         last_calculated_index[current_thread as usize],
    //         tasks_for_thread[current_thread as usize].pop_front().unwrap(),
    //     );
    //     channels.push(current_task.receiver);
    //     threads_handles.push(current_task.start_thread());
    //     calculated_elements += 1;
    // }
    //
    // while calculated_elements < amount_of_elements as u32 {
    //     for current_thread in 0..thread_count {
    //         match channels[current_thread as usize].try_recv() {
    //             Ok(result_from_thread) => {
    //                 result += result_from_thread.0.clone();
    //                 last_calculated_index[current_thread as usize] = result_from_thread.1;
    //                 elements[last_calculated_index[current_thread as usize] as usize] = result_from_thread.0.clone();
    //
    //                 let next_task: u32 = match tasks_for_thread[current_thread as usize].pop_front() {
    //                     Some(next_task) => next_task,
    //                     None => {
    //                         for i in 0..amount_of_work_per_thread {
    //                             match all_tasks.pop_front() {
    //                                 Some(next_task) => {
    //                                     tasks_for_thread[current_thread].push_back(next_task);
    //                                 }
    //                                 None => break
    //                             }
    //                         }
    //                     }
    //                 };
    //
    //                 let current_task = Task::new(
    //                     elements[(current_thread * amount_of_work_per_thread) as usize].clone(),
    //                     last_calculated_index[current_thread as usize],
    //                     next_task,
    //                 );
    //                 channels[current_thread] = current_task.receiver;
    //                 threads_handles[current_thread as usize] = current_task.start_thread();
    //                 calculated_elements += 1;
    //             }
    //             Err(_) => continue
    //         }
    //     }
    // }

    println!("{}", 1 / result);
}
