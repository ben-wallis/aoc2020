use aoc2020::utils::read_lines;
use std::iter::once;

fn main() {
    let input = read_lines("C:\\rust\\aoc2020\\day3_input.txt");

    let map = input
        .map(|line| line.chars().map(|x| x == '#').collect::<Vec<bool>>())
        .collect::<Vec<Vec<bool>>>();

    part1(&map);
    part2(&map);
}

fn part1(map: &Vec<Vec<bool>>) {
    let result = calc_trees(&map, 3, 1);
    println!("total trees: {}", result);
}

fn part2(map: &Vec<Vec<bool>>) {
    let result: u64 = once(calc_trees(&map, 1, 1))
        .chain(once(calc_trees(&map, 3, 1)))
        .chain(once(calc_trees(&map, 5, 1)))
        .chain(once(calc_trees(&map, 7, 1)))
        .chain(once(calc_trees(&map, 1, 2)))
        .product();
    println!("part 2: {}", result);
}

fn calc_trees(map: &Vec<Vec<bool>>, step_x: usize, step_y: usize) -> u64 {
    let mut x = 0;
    let mut y = 0;
    let mut trees = 0;

    while y < map.len() {
        trees += map[y][x % map[0].len()] as u32;
        x += step_x;
        y += step_y;
    }

    trees as u64
}
