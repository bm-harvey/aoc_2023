/// Day 1 function for advent of code
/// ```
/// let file_path = "test_data/day_1.dat";
/// let data = std::fs::read_to_string(file_path).expect("Problem reading file to string.");
/// ```
pub fn day_1(raw_data: &str) -> (String, String) {
    (day_1_part_1(raw_data), day_1_part_2(raw_data))
}

/// Day 1 function for advent of code
/// ```
/// let file_path = "test_data/day_1.dat";
/// let data = std::fs::read_to_string(file_path).expect("Problem reading file to string.");
/// println!("{data}");
/// assert_eq!(aoc_2023::day_1::day_1_part_1(&data), String::from("142"));
/// ```
pub fn day_1_part_1(raw_data: &str) -> String {
    let total: u32 = raw_data
        .lines()
        .map(|line| {
            let numbers = line.chars().filter(|c| c.is_numeric()).collect::<Vec<_>>();
            debug_assert!(!numbers.is_empty());

            let digit_1 = numbers.first().unwrap().to_digit(10).unwrap();
            let digit_2 = numbers.last().unwrap().to_digit(10).unwrap();

            10 * digit_1 + digit_2
        })
        .sum();

    format!("{total}")
}

/// Day 1 function for advent of code
/// ```
/// let file_path = "test_data/day_1_part2.dat";
/// let data = std::fs::read_to_string(file_path).expect("Problem reading file to string.");
/// println!("{data}");
/// assert_eq!(aoc_2023::day_1::day_1_part_2(&data), String::from("281"));
/// ```
pub fn day_1_part_2(_raw_data: &str) -> String {
    let process_line = |line: &str| {
        let mut idx_l = 0;
        let mut calibration_numbers = Vec::<u32>::new();
        'inf: loop {
            if idx_l >= line.len() {
                break;
            }

            let letter = line.chars().nth(idx_l).unwrap();
            if letter.is_numeric() {
                calibration_numbers.push(letter.to_digit(10).unwrap());
                idx_l += 1;
                continue;
            }

            for idx_r in (idx_l + 2)..std::cmp::min(line.len(), idx_l + 5) {
                let word = &line[idx_l..=idx_r];
                let number = get_number(word);

                if let Some(num) = number {
                    calibration_numbers.push(num);
                    idx_l = idx_r + 1;
                    continue 'inf;
                }
            }

            idx_l += 1;
        }
        10 * calibration_numbers.first().unwrap() + calibration_numbers.last().unwrap()
    };

    let total: u32 = _raw_data.lines().map(process_line).sum();

    format!("{total}")
}

fn get_number(word: &str) -> Option<u32> {
    if word == "one" {
        Some(1)
    } else if word == "two" {
        Some(2)
    } else if word == "three" {
        Some(3)
    } else if word == "four" {
        Some(4)
    } else if word == "five" {
        Some(5)
    } else if word == "six" {
        Some(6)
    } else if word == "seven" {
        Some(7)
    } else if word == "eight" {
        Some(8)
    } else if word == "nine" {
        Some(9)
    } else {
        None
    }
}
