use day_04::part1_intersect::process;

fn main() {
    let file = include_str!("../../input1.txt");
    let result = process(file);
    println!("{result}");
}
