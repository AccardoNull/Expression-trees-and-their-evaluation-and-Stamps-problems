// Starter file provided to CSC 330, Spring 2025, Assignment 3
// Copyright Mike Zastre, UVic 2025.
//
// This echoes the functionality provided by the starter file in
// Haskell for the similar problem in Assignment 1.
//
// Therefore your task is to complete the functionality needed
// by `max_coverage()` -- and which will (perhaps) including writing
// other Rust functions in turn.
//

use std::fs::{File, read_to_string};
use std::env;
use std::io::{Write};


fn max_coverage(m: usize, denominations: Vec<u32>) -> (u32, Vec<u32>) {
    use std::collections::HashSet;

    fn dfs(val: u32, used: usize, idx: usize, m: usize, denoms: &[u32], seen: &mut HashSet<u32>) {
        if used > m {
            return;
        }
        seen.insert(val);
        (idx..denoms.len()).for_each(|i| {
            dfs(val + denoms[i], used + 1, i, m, denoms, seen)
        });
    }

    let mut seen = HashSet::new();
    dfs(0, 0, 0, m, &denominations, &mut seen);

    let mut count = 0;
    while seen.contains(&(count + 1)) {
        count += 1;
    }

    (count, denominations.clone()) // return original order
}


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <input file> <output file>", args[0]);
        return;
    }

    let contents: String = read_to_string(&args[1])
        .expect("Should have been able to read the file.");

    let mut lines = contents.lines();

    let mut output_file = File::create(&args[2])
        .expect("Failed to created output file");

    while let Some(size_line) = lines.next() {
        let m: usize = size_line.trim()
            .parse()
            .unwrap_or(0);

        if m == 0 {
            break;
        }
        
        if let Some(denoms_line) = lines.next() {
            let mut parts = denoms_line
               .split_whitespace()
               .map(|s| s.parse().unwrap());
            let _count = parts.next(); // skip the first number
            let values: Vec<u32> = parts.collect();

            
            let (max_value, denominations) = max_coverage(m, values);

            writeln!(output_file, "max coverage = {} : {}", max_value, denominations.iter().map(|d| d.to_string()).collect::<Vec<String>>().join(" "))
                .expect("Failed to write to output file.");
        } else {
            eprintln!("Error: Expected a second line, but found none.");
            break;
        }
    }
}