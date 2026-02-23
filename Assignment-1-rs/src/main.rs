#![feature(test)]
extern crate test;

mod sorting;
use rand::prelude::*;

fn main() {
    for i in 1..100 {
        run_merge_sort(i);
    }
}

fn run_selection_sort(size: u32) {
    let mut rng = rand::rng();
    let mut arr: Vec<u32> = vec![];
    for _ in 0..size {
        arr.push(rng.random_range(0..=100));
    }
    sorting::selection_sort(&mut arr);
    //println!("{:04}. Sorted: {}", size, arr.is_sorted());
}

fn run_merge_sort(size: u32) {
    let mut rng = rand::rng();
    let mut arr: Vec<u32> = vec![];
    for _ in 0..size {
        arr.push(rng.random_range(0..=100));
    }
    sorting::merge_sort(&mut arr);
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_selection_sort_10(b: &mut Bencher) {
        b.iter(|| run_selection_sort(10));
    }
    #[bench]
    fn bench_merge_sort_10(b: &mut Bencher) {
        b.iter(|| run_merge_sort(10));
    }

    #[bench]
    fn bench_selection_sort_100(b: &mut Bencher) {
        b.iter(|| run_selection_sort(100));
    }
    #[bench]
    fn bench_merge_sort_100(b: &mut Bencher) {
        b.iter(|| run_merge_sort(100));
    }

    #[bench]
    fn bench_selection_sort_1000(b: &mut Bencher) {
        b.iter(|| run_selection_sort(1000));
    }
    #[bench]
    fn bench_merge_sort_1000(b: &mut Bencher) {
        b.iter(|| run_merge_sort(1000));
    }
}
