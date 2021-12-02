use aoc_runner_derive::{aoc, aoc_generator};

enum Direction {
    Forward,
    Down,
    Up,
    None,
}

impl Direction{
    pub fn matcher(i: &str) -> Direction { 
        match i {
            "forward" => Direction::Forward,
            "down" => Direction::Down,
            "up" => Direction::Up,
            _ => Direction::None,
        }
    }
}

pub struct Command {
    dir: Direction,
    val: i32,
}

impl Command {
    pub fn adjust(&self, depth: i32, horizontal: i32) -> (i32, i32) {
        let (mut d, mut h) = (depth, horizontal);
        match self.dir {
            Direction::Forward => h += self.val,
            Direction::Down => d += self.val,
            Direction::Up => d -= self.val,
            Direction::None => {},
        }
        (d, h)
    }
    
    pub fn adjust_aim(&self, depth: i32, horizontal: i32, aim: i32) -> (i32, i32, i32) {
        let (mut d, mut h, mut a) = (depth, horizontal, aim);
        match self.dir {
            Direction::Forward => {
                h += self.val;
                d += a * self.val;
            },
            Direction::Down => a += self.val,
            Direction::Up => a -= self.val,
            Direction::None => {},
        }
        (d, h, a)
    }
}

#[aoc_generator(day2)]
fn input_generator(input: &str) -> Vec<Command> {
    input.lines().map(|x| {
        let split_line = x.split(' ').collect::<Vec<&str>>();
        Command{dir: Direction::matcher(split_line[0]), val: split_line[1].parse().unwrap()}
    }).collect()
}

#[aoc(day2, part1)]
pub fn day2_part1(commands: &[Command]) -> i32 {
    let tuple = commands.iter().fold((0, 0), |(d, h), command| command.adjust(d, h));

    tuple.0 * tuple.1
}

#[aoc(day2, part2)]
pub fn day2_part2(commands: &[Command]) -> i32 {
    let tuple = commands.iter().fold((0, 0, 0), |(d, h, a), command| command.adjust_aim(d, h, a));

    tuple.0 * tuple.1
}