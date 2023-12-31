use day_05::*;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn part1() {
    part1::process(divan::black_box(include_str!("../input1.txt",)));
}

#[divan::bench(sample_count = 1)]
fn part2() {
    part2::process(divan::black_box(include_str!("../input2.txt",)));
}

#[divan::bench]
fn part2_rev() {
    part2_rev::process(divan::black_box(include_str!("../input2.txt",)));
}
