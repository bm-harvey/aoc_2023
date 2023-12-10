use crate::solution::Solution;
pub struct Day5;

impl Solution for Day5 {
    fn solution(raw_data: &str) -> (Box<dyn std::fmt::Display>, Box<dyn std::fmt::Display>) {
        let mut lines = raw_data.lines();

        let seed_line_values = lines
            .next()
            .unwrap()
            .chars()
            .skip_while(|&c| c != ' ')
            .collect::<String>()
            .split_whitespace()
            .map(|value| value.parse::<u32>().unwrap())
            .collect::<Vec<_>>();

        let almanac = Almanac::new(raw_data);

        let sol_1 = seed_line_values
            .iter()
            .map(|&seed| almanac.location_from_seed(seed))
            .min()
            .unwrap();

        let sol_2 = (0..seed_line_values.len() / 2)
            .flat_map(|pair_idx| {
                seed_line_values[pair_idx * 2]
                    ..seed_line_values[pair_idx * 2] + seed_line_values[pair_idx * 2 + 1]
            })
            .map(|seed| almanac.location_from_seed(seed))
            .min()
            .unwrap();

        (Box::new(sol_1), Box::new(sol_2))
    }
}

#[derive(Default)]
struct IdxRange {
    start_in: u32,
    start_out: u32,
    len: u32,
}

impl IdxRange {
    fn new(start_in: u32, start_out: u32, len: u32) -> Self {
        Self {
            start_in,
            start_out,
            len,
        }
    }

    fn contains(&self, idx: u32) -> bool {
        (self.start_in..self.start_in + self.len).contains(&idx)
    }
    fn next_idx(&self, idx: u32) -> u32 {
        idx - self.start_in + self.start_out
    }
}

#[derive(Default)]
struct PropertyMap {
    ranges: Vec<IdxRange>,
}

impl PropertyMap {
    fn add_range(&mut self, start_in: u32, start_out: u32, len: u32) -> &mut Self {
        self.ranges.push(IdxRange::new(start_in, start_out, len));
        self
    }
    fn add_data(&mut self, data: &str) -> &mut Self {
        let values = data
            .split_whitespace()
            .map(|value| {
                value
                    .parse::<u32>()
                    .expect("Problem parsing input file data")
            })
            .collect::<Vec<_>>();

        if values.len() == 3 {
            self.add_range(values[1], values[0], values[2]);
        }
        self
    }

    fn next_idx(&self, idx: u32) -> u32 {
        let range = self.ranges.iter().find(|range| range.contains(idx));
        match range {
            Some(r) => r.next_idx(idx),
            None => idx,
        }
    }
}

#[derive(Default)]
struct Almanac {
    seed_to_soil: PropertyMap,
    soil_to_fertilizer: PropertyMap,
    fertilizer_to_water: PropertyMap,
    water_to_light: PropertyMap,
    light_to_temperature: PropertyMap,
    temperature_to_humidity: PropertyMap,
    humidity_to_location: PropertyMap,
}

impl Almanac {
    fn new(input: &str) -> Self {
        let mut result = Almanac::default();

        let lines = input.lines().collect::<Vec<_>>();
        let hdr_idxs = lines
            .iter()
            .enumerate()
            .filter_map(|(line_idx, line)| match line.contains("map:") {
                true => Some(line_idx),
                false => None,
            })
            .collect::<Vec<usize>>();

        for line in &lines[hdr_idxs[0] + 1..hdr_idxs[1]] {
            result.seed_to_soil.add_data(line);
        }
        for line in &lines[hdr_idxs[1] + 1..hdr_idxs[2]] {
            result.soil_to_fertilizer.add_data(line);
        }
        for line in &lines[hdr_idxs[2] + 1..hdr_idxs[3]] {
            result.fertilizer_to_water.add_data(line);
        }
        for line in &lines[hdr_idxs[3] + 1..hdr_idxs[4]] {
            result.water_to_light.add_data(line);
        }
        for line in &lines[hdr_idxs[4] + 1..hdr_idxs[5]] {
            result.light_to_temperature.add_data(line);
        }
        for line in &lines[hdr_idxs[5] + 1..hdr_idxs[6]] {
            result.temperature_to_humidity.add_data(line);
        }
        for line in &lines[hdr_idxs[6] + 1..] {
            result.humidity_to_location.add_data(line);
        }

        result
    }

    fn location_from_seed(&self, seed: u32) -> u32 {
        let soil = self.seed_to_soil.next_idx(seed);
        let fertilizer = self.soil_to_fertilizer.next_idx(soil);
        let water = self.fertilizer_to_water.next_idx(fertilizer);
        let light = self.water_to_light.next_idx(water);
        let temperature = self.light_to_temperature.next_idx(light);
        let humidity = self.temperature_to_humidity.next_idx(temperature);
        self.humidity_to_location.next_idx(humidity)
    }
}
