use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::{ErrorKind,Error};

fn isqrt(n: usize) -> usize {
    if n == 0 {
        return n;
    }
    let mut s = (n as f64).sqrt() as usize;
    s = (s + n / s) >> 1;
    if s * s > n {
        return s - 1 
    } 
    return s 
}

fn best_sqrt(n: usize) -> isize {
    match n & 0xf {
        0 | 1 | 4 | 9 => {
            let t = isqrt(n);
            if t*t == n { 
               return t as isize
            }
        },
        _ => {}
    }
    return -1
}

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect::<Vec<String>>();
    let filename: &str = args.get(1)
        .ok_or(Error::new(ErrorKind::Other, "No filename given"))?;
    let mut file: File = File::open(filename)?;
    let mut contents: String = String::new();
    file.read_to_string(&mut contents)?;
    let lines: Vec<&str> = contents.split("\n").collect();

    // Part 1, numbers are split.
    // let times: Vec<usize> = lines[0].split(" ")
    //     .filter_map(|x| x.parse::<usize>().ok())
    //     .collect::<Vec<usize>>();
    // let distances: Vec<usize> = lines[1].split(" ")
    //     .filter_map(|x| x.parse::<usize>().ok())
    //     .collect::<Vec<usize>>();

    // Part 2, join all numbers into single integer.
    let times: usize = lines[0].split(" ")
        .filter_map(|x| x.parse::<usize>().ok())
        .fold(0, |acc, x| acc * 10_usize.pow(f32::log10(x as f32).ceil() as u32) + x);
    let distances: usize = lines[1].split(" ")
        .filter_map(|x| x.parse::<usize>().ok())
        .fold(0, |acc, x| acc * 10_usize.pow(f32::log10(x as f32).ceil() as u32) + x);
    
    // To reuse the same code, we just put the single integer into vectors...
    let times: Vec<usize> = vec![times];
    let distances: Vec<usize> = vec![distances];

    if times.len() != distances.len() {
        return Err(Error::new(ErrorKind::Other, "Times and distances are not the same length"));
    }
    for i in 0..times.len() {
        println!("{} {}", times[i], distances[i]);
    }

    // Find x for which x^2 - Tx + D = 0.
    // x_left = (T - sqrt(T^2 - 4D)) / 2, x_right = (T + sqrt(T^2 - 4D)) / 2
    // Find all integers on the interval [x_left, x_right] for each (T,D) pair.
    // Print the product of the number of integers in each interval.
    
    let mut slns: Vec<usize> = Vec::new();
    for i in 0..times.len() {
        let t: usize = times[i];
        let d: usize = distances[i];
        let int_sqrt: isize = best_sqrt(t.pow(2) - 4*d);
        if int_sqrt >= 0 {
            let int_sqrt: usize = int_sqrt as usize;
            let x_left: usize = if (t - int_sqrt) % 2 == 0 { (t - int_sqrt) / 2 + 1 } else { (t - int_sqrt + 1) / 2 };
            let x_right: usize = if (t + int_sqrt) % 2 == 0 { (t + int_sqrt) / 2 - 1 } else { (t + int_sqrt - 1) / 2 };
            slns.push(x_right - x_left + 1);
            continue;
        }
        let x_left: f64 = (t as f64 - (t.pow(2) as f64 - 4.0*d as f64).sqrt()) / 2.0;
        let x_right: f64 = (t as f64 + (t.pow(2) as f64 - 4.0*d as f64).sqrt()) / 2.0;
        slns.push((x_right.floor() - x_left.ceil() + 1.0) as usize);
    }
        
    println!("{}", slns.iter().fold(1, |acc, x| acc * x));
    return Ok(());
}
