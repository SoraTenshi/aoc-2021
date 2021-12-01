use aoc_runner_derive::aoc;

#[aoc(day1, part1)]
pub fn day1_part1(input: &str) -> usize {
    let depths: Vec<u32> = input.lines().map(|x| x.parse().unwrap()).collect();
    
    depths.windows(2).filter(|x| x[0] < x[1]).count()
}

#[aoc(day1, part2)]
pub fn day1_part2(input: &str) -> usize {
    let depths: Vec<u32> = input.lines().map(|x| x.parse().unwrap()).collect();

    let summarized: Vec<u32> = depths.windows(3).map(|x| x[0] + x[1] + x[2]).collect();

    summarized.windows(2).filter(|x| x[0] < x[1]).count()
}
