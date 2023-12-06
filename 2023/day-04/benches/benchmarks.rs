use day_04::*;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn part1() {
    part1::process(divan::black_box(include_str!("../input1.txt",)));
}

#[divan::bench]
fn part1_intersect() {
    part1_intersect::process(divan::black_box(include_str!("../input1.txt",)));
}

#[divan::bench]
fn part2() {
    part2::process(divan::black_box(include_str!("../input2.txt",)));
}

#[divan::bench]
fn part2_intersect() {
    part2_intersect::process(divan::black_box(include_str!("../input2.txt",)));
}

#[divan::bench]
fn part2_cache() {
    part2_cache::process(divan::black_box(include_str!("../input2.txt",)));
}

#[divan::bench]
fn part2_cache2() {
    part2_cache2::process(divan::black_box(include_str!("../input2.txt",)));
}
