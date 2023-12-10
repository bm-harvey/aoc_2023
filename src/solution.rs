use std::fmt::Display;

pub trait Solution {
    fn solution(raw_data: &str) -> (Box<dyn Display>, Box<dyn Display>);

    fn print(raw_data: &str) {
        let sol = Self::solution(raw_data);
        println!("Solution 1: {}", sol.0);
        println!("Solution 2: {}", sol.1);
    }
}
