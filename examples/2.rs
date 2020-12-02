use aoc2020::utils::read_lines;
use regex::Regex;
use std::ops::Range;

fn main() {
    part1();
    //part2();
}

fn part1() {
    let regex = Regex::new(r"(\d.*)-(\d.*)\s(\w): (\w.*)").unwrap();

    let result = read_lines("C:\\rust\\aoc2020\\day2_input.txt")
        .filter(|line| {
            let captures = regex.captures(&line).unwrap();
            let range = captures[1].parse::<u32>().unwrap()..=captures[2].parse::<u32>().unwrap();
            let char = captures[3].to_owned();
            let input = captures[4].to_owned();
            let occurances = input.matches(&char).count() as u32;
            range.contains(&occurances)
        })
        .count();

    println!("{}", result);
}
fn part2() {
    let regex = Regex::new(r"(\d.*)-(\d.*)\s(\w): (\w.*)").unwrap();

    let result = read_lines("C:\\rust\\aoc2020\\day2_input.txt")
        .map(|line| {
            let captures = regex.captures(&line).unwrap();
            let a: usize = captures[1].parse().unwrap();
            let b: usize = captures[2].parse().unwrap();

            let char = captures[3].to_owned().chars().next().unwrap();
            let input = captures[4].to_owned();

            let check_pos = |pos: usize| input.chars().nth(pos - 1).map_or(false, |x| x == char);
            check_pos(a) ^ check_pos(b)
        })
        .filter(|x| *x)
        .count();

    println!("{}", result);
}
