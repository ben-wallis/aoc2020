use aoc2020::utils::read_lines;

fn main() {
    let input: Vec<u32> = read_lines("C:\\rust\\aoc2020\\day1_input.txt")
        .map(|x| x.parse::<u32>().unwrap())
        .collect();
    for i in input.clone() {
        for j in input.clone() {
            for k in input.clone() {
                if i + j + k == 2020 {
                    let answer = i * j * k;
                    println!("The answer is: {}", answer);
                    break;
                }
            }
        }
    }
}
