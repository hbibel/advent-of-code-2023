use std::{collections::HashMap, ops::Range};

use regex::Regex;

#[derive(Debug)]
struct GardenMap {
    from: String,
    to: String,
    ranges: Vec<GardenRange>,
}

#[derive(Debug)]
struct GardenRange {
    key_start: i64,
    key_end: i64,
    value_start: i64,
}

impl GardenMap {
    fn get(&self, key: i64) -> i64 {
        self.ranges
            .iter()
            .find(|range| key >= range.key_start && key <= range.key_end)
            .map(|range| range.value_start + key - range.key_start)
            .unwrap_or(key)
    }

    fn from_lines(lines: &Vec<String>) -> GardenMap {
        let header_line = lines.get(0).unwrap();
        let map_re = Regex::new(r"(\w+)-to-(\w+) map:").unwrap();
        let (from, to) = map_re
            .captures(header_line)
            .map(|capture| {
                let (_, [f, t]) = capture.extract();
                (f.to_string(), t.to_string())
            })
            .unwrap();

        let ranges = lines
            .iter()
            .skip(1)
            .map(|line| {
                let nums: Vec<&str> = line.split_whitespace().collect();
                let key_start = nums.get(1).unwrap().parse::<i64>().unwrap();
                let key_end = key_start + nums.get(2).unwrap().parse::<i64>().unwrap() - 1;
                let value_start = nums.get(0).unwrap().parse::<i64>().unwrap();
                GardenRange {
                    key_start,
                    key_end,
                    value_start,
                }
            })
            .collect();
        GardenMap { from, to, ranges }
    }
}

pub fn lowest_location(input: String) -> i64 {
    let mut line_iter = input.lines().map(|l| l.trim()).filter(|l| !l.is_empty());

    let seeds: Vec<i64> = line_iter
        .next()
        // strip prefix "seeds: "
        .map(|line| line.split_whitespace().skip(1))
        .map(|seed_ids| seed_ids.map(|s| s.parse::<i64>().unwrap()))
        .unwrap()
        .collect();

    let mut map_lines: Vec<String> = Vec::new();
    let mut lookup = HashMap::new();
    while let Some(line) = line_iter.next() {
        if !map_lines.is_empty() && line.chars().nth(0).unwrap().is_alphabetic() {
            // line looks like "foo-to-bar map:"
            let gm = GardenMap::from_lines(&map_lines);
            lookup.insert(gm.from.clone(), gm);
            map_lines.clear();
            map_lines.push(line.to_string());
        } else {
            map_lines.push(line.to_string())
        }
    }
    let gm = GardenMap::from_lines(&map_lines);
    lookup.insert(gm.from.clone(), gm);

    let mut locations: Vec<i64> = Vec::new();
    let target = String::from("location");
    seeds.iter().for_each(|seed| {
        let mut key = *seed;
        let mut key_name = String::from("seed");

        while key_name != target {
            let gm = lookup.get(&key_name).unwrap();
            key = gm.get(key);
            key_name = gm.to.clone();
        }
        locations.push(key);
    });
    *locations.iter().min().unwrap()
}

pub fn lowest_location_ranges(input: String) -> i64 {
    let mut line_iter = input.lines().map(|l| l.trim()).filter(|l| !l.is_empty());

    let seeds: Vec<i64> = line_iter
        .next()
        // strip prefix "seeds: "
        .map(|line| line.split_whitespace().skip(1))
        .map(|seed_ids| {
            let seed_ids: Vec<&str> = seed_ids.collect();
            seed_ids
                .iter()
                .step_by(2)
                .zip(seed_ids.iter().skip(1).step_by(2))
                .flat_map(|(start_str, count_str)| {
                    let start = start_str.parse::<i64>().unwrap();
                    let end = count_str.parse::<i64>().unwrap() + start - 1;

                    Range { start, end }
                })
                .collect::<Vec<i64>>()
        })
        .unwrap();

    // lines below are just copied from above
    let mut map_lines: Vec<String> = Vec::new();
    let mut lookup = HashMap::new();
    while let Some(line) = line_iter.next() {
        if !map_lines.is_empty() && line.chars().nth(0).unwrap().is_alphabetic() {
            // line looks like "foo-to-bar map:"
            let gm = GardenMap::from_lines(&map_lines);
            lookup.insert(gm.from.clone(), gm);
            map_lines.clear();
            map_lines.push(line.to_string());
        } else {
            map_lines.push(line.to_string())
        }
    }
    let gm = GardenMap::from_lines(&map_lines);
    lookup.insert(gm.from.clone(), gm);

    let mut locations: Vec<i64> = Vec::new();
    let target = String::from("location");
    seeds.iter().for_each(|seed| {
        let mut key = *seed;
        let mut key_name = String::from("seed");

        while key_name != target {
            let gm = lookup.get(&key_name).unwrap();
            key = gm.get(key);
            key_name = gm.to.clone();
        }
        locations.push(key);
    });
    *locations.iter().min().unwrap()
}

// Note: While the brute force solution above was running, I was working on
// this alternative solution using range intersection. However the brute force
// computation completed before I was finished with the alternative.
//
// // all numbers between from and to are modified by offset
// struct RangeMod {
//     from: i64,
//     to: i64,
//     offset: i64,
// }
//
// fn alternative(input: String) -> u64 {
//     let mut line_iter = input.lines().map(|l| l.trim()).filter(|l| !l.is_empty());
//
//     let mut ranges: Vec<(i64, i64)> = line_iter
//         .next()
//         // strip prefix "seeds: "
//         .map(|line| line.split_whitespace().skip(1))
//         .map(|seed_ids| {
//             seed_ids
//                 .collect::<Vec<_>>()
//                 .chunks_exact(2)
//                 .map(|chunk| {
//                     let from = chunk[0].parse::<i64>().unwrap();
//                     let to = chunk[1].parse::<i64>().unwrap();
//                     (from, to)
//                 })
//                 .collect()
//         })
//         .unwrap();
//
//     let mut map_markers: Vec<usize> = input
//         .lines()
//         .enumerate()
//         .filter_map(
//             |(i, line)| {
//                 if line.contains("map:") {
//                     Some(i)
//                 } else {
//                     None
//                 }
//             },
//         )
//         .collect();
//     map_markers.push(input.lines().count());
//
//     // Chunks of input that describe one map
//     let map_sections = map_markers
//         .iter()
//         .step_by(2)
//         .zip(map_markers.iter().skip(1).step_by(2))
//         .map(|(&start, &end)| {
//             input
//                 .lines()
//                 .skip(start)
//                 .take(end - start)
//                 .map(|s| s.to_string())
//                 .collect::<Vec<String>>()
//         });
//
//     map_sections.for_each(|map_lines| {
//         let mods = map_lines.iter().map(|line| {
//             let parts = line.split_whitespace().collect::<Vec<_>>();
//             let from = parts.get(1).unwrap().parse::<i64>().unwrap();
//             let to = from + parts.get(2).unwrap().parse::<i64>().unwrap();
//             let offset = parts.get(0).unwrap().parse::<i64>().unwrap() - from;
//             RangeMod { from, to, offset }
//         });
//
//         // TODO
//         // ranges = ranges.flatmap ( range =>
//         //  updated = [range]
//         //  for mod in mods {
//         //    if mod applies to range {
//         //      updated = mod.apply(updated)
//         //    }
//         //  }
//     });
//     todo()
// }

#[cfg(test)]
mod tests {
    use super::lowest_location;
    use super::lowest_location_ranges;

    #[test]
    fn example_1() {
        let input = String::from(
            "seeds: 79 14 55 13
 
             seed-to-soil map:
             50 98 2
             52 50 48
 
             soil-to-fertilizer map:
             0 15 37
             37 52 2
             39 0 15
 
             fertilizer-to-water map:
             49 53 8
             0 11 42
             42 0 7
             57 7 4
 
             water-to-light map:
             88 18 7
             18 25 70
 
             light-to-temperature map:
             45 77 23
             81 45 19
             68 64 13
 
             temperature-to-humidity map:
             0 69 1
             1 0 69
 
             humidity-to-location map:
             60 56 37
             56 93 4",
        );
        let actual = lowest_location(input);
        let expected = 35;
        assert_eq!(actual, expected);
    }

    #[test]
    fn example_2() {
        let input = String::from(
            "seeds: 79 14 55 13
 
             seed-to-soil map:
             50 98 2
             52 50 48
 
             soil-to-fertilizer map:
             0 15 37
             37 52 2
             39 0 15
 
             fertilizer-to-water map:
             49 53 8
             0 11 42
             42 0 7
             57 7 4
 
             water-to-light map:
             88 18 7
             18 25 70
 
             light-to-temperature map:
             45 77 23
             81 45 19
             68 64 13
 
             temperature-to-humidity map:
             0 69 1
             1 0 69
 
             humidity-to-location map:
             60 56 37
             56 93 4",
        );
        let actual = lowest_location_ranges(input);
        let expected = 46;
        assert_eq!(actual, expected);
    }
}
