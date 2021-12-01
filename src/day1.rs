use aoc_runner_derive::aoc;

#[aoc(day1, part1)]
pub fn day1(input: &str) -> u32 {
    let mut counter = 0u32;
    let mut last_depth = 0u32;

    for depth in input.lines() {
        let cur_depth = depth.parse().unwrap();
        if last_depth > 0 && cur_depth > last_depth {
            counter += 1u32;
        }
        last_depth = cur_depth;
    }
    counter
}