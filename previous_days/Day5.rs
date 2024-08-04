use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{ErrorKind,Error};
use std::io::prelude::*;

#[derive(Clone, Debug)]
struct LookupRange {
    starting_index: usize,
    length: usize,
}

impl LookupRange {
    fn new(starting_index: usize, length: usize) -> LookupRange {
        LookupRange {
            starting_index: starting_index,
            length: length,
        }
    }
}

// Assume lookup_ranges is sorted by starting_index.
fn expand_lr(lr: &mut LookupRange, lookup_ranges: &Vec<(LookupRange, usize)>) -> Vec<LookupRange> {
    let mut new_lookup_ranges: Vec<LookupRange> = Vec::new();
    for (lookup_range, output_index) in lookup_ranges {
        if lr.length == 0{
            break;
        }
        if lr.starting_index >= lookup_range.starting_index + lookup_range.length {
            continue;
        }
        if lr.starting_index < lookup_range.starting_index && lr.starting_index + lr.length - 1 < lookup_range.starting_index {
            new_lookup_ranges.push(LookupRange::new(
                    lr.starting_index,
                    lr.length,
            ));
            lr.starting_index = lr.starting_index + lr.length;
            lr.length = 0;
            break;
        }
        if lr.starting_index < lookup_range.starting_index && lr.starting_index + lr.length - 1 >= lookup_range.starting_index {
            new_lookup_ranges.push(LookupRange::new(
                    lr.starting_index,
                    lookup_range.starting_index - lr.starting_index,
            ));
            lr.length = lr.length - (lookup_range.starting_index - lr.starting_index);
            lr.starting_index = lookup_range.starting_index;
        }
        let num_elts: usize = [lr.length, lookup_range.length - (lr.starting_index - lookup_range.starting_index)].iter().min().unwrap().clone();
        let new_lr: LookupRange = LookupRange::new(
            *output_index + (lr.starting_index - lookup_range.starting_index),
            num_elts,
        );
        lr.length = lr.length - num_elts;
        lr.starting_index = lr.starting_index + num_elts;
        new_lookup_ranges.push(new_lr);
    }
    if lr.length > 0 {
        new_lookup_ranges.push(LookupRange::new(
                lr.starting_index,
                lr.length,
        ));
    }
    return new_lookup_ranges;
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect::<Vec<String>>();
    let filename: &str = args.get(1)
        .ok_or(Error::new(ErrorKind::Other, "No filename given"))?;
    let mut file: File = File::open(filename)?;
    let mut contents: String = String::new();
    file.read_to_string(&mut contents)?;
    let lines: Vec<&str> = contents.split("\n").collect();

    let mut almanac_types:
        HashMap<(&str,&str), Vec<(LookupRange, usize)>> = HashMap::new();

    // Get seeds from first line.
    // First element is "seeds:".
    let seeds: Vec<&str> = lines[0].split(" ").collect();
    if seeds.len() == 0 {
        return Err(Error::new(ErrorKind::Other, "No seeds given"));
    }
    let mut lrs: Vec<LookupRange> = Vec::new();
    for index in 0..seeds.len()/2 {
        lrs.push(LookupRange::new(
            seeds[2*(index+1)-1].parse::<usize>().unwrap(),
            seeds[2*(index+1)].parse::<usize>().unwrap(),
        ));
    }
    let mut key_iteration_order: HashMap<&str,&str> = HashMap::new();
    let mut from: &str = "";
    let mut to: &str = "";
    for line in lines {
        let line: Vec<&str> = line.split(" ").collect();
        // Lines containing "map:" are the start of a new mapping.
        if line.len() == 2 && line[1] == "map:" {
            let line: Vec<&str> = line[0].split("-").collect();
            if line.len() == 3 && line[1] == "to" {
                from = line[0];
                to = line[2];
                key_iteration_order.insert(from, to);
                almanac_types.insert((from, to), Vec::new());
            }
        } else if line.len() == 3 { 
            // Panic if we have not seen the keys before.
            almanac_types.get_mut(&(from, to)).unwrap().push((LookupRange::new(
                line[1].parse::<usize>().unwrap(),
                line[2].parse::<usize>().unwrap(),
            ), line[0].parse::<usize>().unwrap()));
        }
    }

    let mut seed_to_location: Vec<&str> = vec![""; key_iteration_order.len()+1];
    // Determine the path from seed to location.
    // Should be seed -> ... -> location.
    // We do this by iterating over the keys in the map constructing a tuple
    // representing the sequence of mappings to get from seed to location.
    seed_to_location[0] = "seed";
    for index in 1..=key_iteration_order.len() {
        seed_to_location[index] = key_iteration_order[seed_to_location[index-1]];
        almanac_types.get_mut(&(seed_to_location[index-1], seed_to_location[index])).unwrap().sort_by(|a, b| a.0.starting_index.partial_cmp(&b.0.starting_index).unwrap());
    }


    // Iterate over the seeds that we need to plant to find its planting
    // location.
    for index in 0..seed_to_location.len()-1 {
        let from: &str = seed_to_location[index];
        let to: &str = seed_to_location[index+1];
        let mut new_lrs: Vec<LookupRange> = Vec::new();
        for mut lr in lrs {
            new_lrs.append(&mut expand_lr(&mut lr, &almanac_types.get(&(from, to)).unwrap()));
        }
        lrs = new_lrs;
    }
    lrs.sort_by(|a, b| a.starting_index.partial_cmp(&b.starting_index).unwrap());
    println!("{}", lrs[0].starting_index);
    Ok(())
}

