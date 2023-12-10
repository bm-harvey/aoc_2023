use crate::solution::Solution;

use std::{collections::HashMap, collections::HashSet};

type NodeMap = HashMap<String, (String, String)>;

pub struct Day8;
impl Solution for Day8 {
    fn solution(raw_data: &str) -> (Box<dyn std::fmt::Display>, Box<dyn std::fmt::Display>) {
        let mut lines = raw_data.lines();
        let lr_instructions = lines.next().unwrap().chars().collect::<Vec<_>>();
        lines.next(); // blank line

        let mut points = NodeMap::new();

        for line in lines {
            let strings = line
                .chars()
                .filter(|&c| c.is_alphabetic() || c.is_numeric() || c == ' ')
                .collect::<String>();
            let strings = strings.split_whitespace().collect::<Vec<_>>();
            points.insert(strings[0].into(), (strings[1].into(), strings[2].into()));
        }

        let mut trys = 0;
        let mut current_node: String = String::from("AAA");
        while current_node != "ZZZ" {
            let move_instruction = lr_instructions[trys % lr_instructions.len()];
            current_node = match move_instruction {
                'L' => points[&current_node].0.clone(),
                'R' => points[&current_node].1.clone(),
                _ => panic!("Invalid LR Instruction"),
            };
            trys += 1;
        }
        let sol_1 = trys as u32;
        dbg!(sol_1);
        let sol_1 = 0;

        let starting_nodes = points
            .iter()
            .filter(|(k, _)| k.ends_with('A'))
            .map(|(k, _)| k.clone())
            .collect::<Vec<_>>();

        // assumes the outputs are relatively prime
        //let sol_2 = starting_nodes
        let mut prime_set = HashSet::<usize>::new();

        let prime_factorizations = starting_nodes
            .into_iter()
            .map(|node| period(&points, &node, &lr_instructions))
            .map(|period| {
                println!("{period}");
                let primes = primes(period);
                for p in primes.iter() {
                    println!("{p}");
                    prime_set.insert(*p);
                }
                primes
            })
            .collect::<Vec<_>>();

        let mut lcm: usize = 1;
        for p in prime_set.iter() {
            let max_count = prime_factorizations
                .iter()
                .map(|set| set.iter().filter(|&x| x == p).count())
                .max()
                .unwrap();
            lcm *= p.checked_pow(max_count as u32).unwrap();
        }

        println!("{lcm}");
        let sol_2 = lcm;
        (Box::new(sol_1), Box::new(sol_2))
    }
}

fn period(points: &NodeMap, entry: &str, lr_dir: &[char]) -> usize {
    let mut node = String::from(entry);
    for step_idx in 1.. {
        let lr_idx = (step_idx - 1) % lr_dir.len();

        let move_dir = lr_dir[lr_idx];
        node = match move_dir {
            'L' => points[&node].0.clone(),
            'R' => points[&node].1.clone(),
            _ => panic!("Invalid LR Instruction"),
        };

        if node.ends_with('Z') {
            return step_idx;
        }
    }
    0
}

fn primes(mut num: usize) -> Vec<usize> {
    let mut result = Vec::new();

    'inf: loop {
        for d in 2..=((num as f64).sqrt().ceil()) as usize {
            if num % d == 0 {
                num /= d;
                result.push(d);
                continue 'inf;
            }
        }

        if num > 1 {
            result.push(num)
        }
        break;
    }
    result
}
