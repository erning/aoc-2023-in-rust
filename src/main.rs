use std::env;
use std::fmt::Display;
use std::time::SystemTime;

fn main() {
    macro_rules! puzzle {
        ($mod:ident, $title:expr) => {
            (
                $title,
                |input| Box::new(aoc::$mod::part_one(input)),
                |input| Box::new(aoc::$mod::part_two(input)),
            )
        };
    }

    type SolverFn = fn(&str) -> Box<dyn Display>;

    let puzzles: Vec<(&str, SolverFn, SolverFn)> = vec![
        puzzle!(day01, "Trebuchet?!"),
        puzzle!(day02, "Cube Conundrum"),
        puzzle!(day03, "Gear Ratios"),
        puzzle!(day04, "Scratchcards"),
    ];

    let filename = match env::args().find(|a| a == "--example") {
        None => "input",
        Some(_) => "example",
    };

    let show_time = env::args().any(|a| a == "--time");

    let mut days: Vec<usize> =
        env::args().filter_map(|a| a.parse().ok()).collect();

    if days.is_empty() {
        days = (1..=puzzles.len()).collect();
    }

    for day in days {
        let (title, part1, part2) = &puzzles[day - 1];
        let input = aoc::read_as_string(day as u8, filename);
        let input = input.as_str();

        println!("--- Day {}: {} ---", day, title);
        let t0 = SystemTime::now();
        println!("Part One: {}", part1(input));
        let t1 = SystemTime::now();
        println!("Part Two: {}", part2(input));
        let t2 = SystemTime::now();

        if show_time {
            let d1 = t1.duration_since(t0).unwrap_or_default();
            let d2 = t2.duration_since(t1).unwrap_or_default();
            println!("Duration: {:?}", (d1, d2));
        }
        println!();
    }
}
