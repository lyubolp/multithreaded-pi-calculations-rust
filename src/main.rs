mod calc;

use crate::calc::pi_calculator::{calculate_a_n_from_previous, calculate_a_n_from_formula};
use rug::{Float};
use std::env;
use std::thread;
use std::thread::JoinHandle;
use std::collections::VecDeque;
use std::sync::{mpsc};
use std::ptr::null;

fn handle_args(args: Vec<String>) -> (usize, usize)
{
    let precision_default: u32 = 10000;
    //let max_thread = num_cpus::get();
    let max_thread = 2;
    /*let max_thread = match cpuid::identify() {
        Ok(info) => info.num_logical_cpus,
        Err(err) => {
            println!("Error getting cpu info, default is 1");
            1
        }
    };*/

    let mut result = (0usize, 0usize);
    if args.len() == 5 {
        if args[1] == String::from("-p")
        {
            match args[2].parse::<usize>() {
                Ok(value) => result.0 = value,
                Err(_) => {
                    println!("Invalid argument for -p, setting it to default");
                    result.0 = precision_default as usize;
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
                    result.0 = precision_default as usize;
                }
            }
        }
        result.1 = max_thread;
    } else {
        println!("Invalid command line arguments, setting the default values (-p {} -t {})", precision_default, max_thread);
        result.0 = precision_default as usize;
        result.1 = max_thread;
    }
    result
}

fn main() {
    let default_precision: u32 = 10000;
    let (amount_of_elements, thread_count) = handle_args(env::args().collect());

    println!("Starting the program with {} elements to be calculated on {} threads", amount_of_elements, thread_count);

    let mut result = Float::with_val(default_precision, 0);
    let mut elements:Vec<Float> = vec![Float::new(default_precision); amount_of_elements as usize];
    let mut tasks: Vec<VecDeque<u32>> = vec![VecDeque::new(); thread_count];

    let amount_of_work_per_thread = {
        if amount_of_elements % thread_count != 0{
            (amount_of_elements / thread_count) + 1
        } else {
            (amount_of_elements / thread_count)
        }
    };



    let mut last_calculated_index: Vec<u32> = vec![0; thread_count];
    let mut calculated_elements: u32 = 0;

    for current_thread in 0..thread_count{
        elements[(amount_of_work_per_thread * current_thread) as usize] =
            calculate_a_n_from_formula( (amount_of_work_per_thread * current_thread) as u32,default_precision);

        last_calculated_index[current_thread] = (amount_of_work_per_thread * current_thread) as u32;
        calculated_elements += 1;
        result += elements[(amount_of_work_per_thread * current_thread) as usize].clone();
    }

    for current_thread in 0..thread_count{
        for i in 1..amount_of_work_per_thread{
            tasks[current_thread as usize].push_back(((amount_of_work_per_thread * current_thread) as u32 + i as u32));
        }
    }

    let mut threads_handles: Vec<JoinHandle<()>> = vec!();
    let mut channels: Vec<mpsc::Receiver<(Float, u32)>> = vec!();
    for current_thread in 0..thread_count{
        let (sender, receiver) = mpsc::channel();
        channels.push(receiver);

        let previous_element = elements[(current_thread * amount_of_work_per_thread) as usize].clone();
        let previous_index: u32 = last_calculated_index[current_thread as usize].clone();
        let target_index = tasks[current_thread as usize].pop_front().unwrap().clone();
        threads_handles.push(thread::spawn(move ||{
            calculate_a_n_from_previous(&previous_element,
                                        &previous_index,
                                        &target_index,
                                        &default_precision,
                                        sender.clone())
        }));
        calculated_elements += 1;
    }

    while calculated_elements < amount_of_elements as u32{
        for current_thread in 0..thread_count{
            match channels[current_thread as usize].try_recv(){
                Ok(result_from_thread) => {
                    result += result_from_thread.0.clone();
                    last_calculated_index[current_thread as usize] = result_from_thread.1;
                    elements[last_calculated_index[current_thread as usize] as usize] = result_from_thread.0.clone();

                    calculated_elements += 1;

                    let previous_a = elements[(current_thread * amount_of_work_per_thread) as usize].clone();
                    let previous_index = last_calculated_index[current_thread as usize];
                    match tasks[current_thread as usize].pop_front(){
                        Some(next_task) =>{
                            let (sender, receiver) = mpsc::channel();
                            channels[current_thread] = receiver;
                            threads_handles[current_thread as usize] = thread::spawn(move ||{
                                calculate_a_n_from_previous(&previous_a,
                                                            &previous_index,
                                                            &next_task,
                                                            &default_precision,
                                                            sender.clone())
                            })
                        }
                        None => continue
                    }
                }
                Err(_) => continue
            }
        }
    }

    println!("{}", 1 / result);
}
