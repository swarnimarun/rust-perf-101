#![feature(test)]

mod conversation;

use test::Bencher;
extern crate test;

fn mut_in_place(x: &mut Vec<i32>) {
    x.push(123);
    x.push(124);
    x.push(124);
    x.push(124);
    x.push(124);
    x.push(125);
    x.push(125);
    x.push(125);
    x.push(125);
    x.push(125);
    x.push(125);
}

#[bench]
fn mutate_attempt1(bench: &mut test::Bencher) {
    let y = vec![1, 2];
    bench.iter(|| {
        let mut t = y.clone();
        mut_in_place(&mut t);
        mut_in_place(&mut t);
        mut_in_place(&mut t);
        mut_in_place(&mut t);
        mut_in_place(&mut t);
        mut_in_place(&mut t);
        mut_in_place(&mut t);
        mut_in_place(&mut t);
        mut_in_place(&mut t);
        mut_in_place(&mut t);
        // println!("{t:?}");
    });
}

#[bench]
fn move_attempt2(bench: &mut test::Bencher) {
    let x = vec![1, 2];
    bench.iter(|| {
        let mut _t = x.clone();
        _t = move_and_append(_t);
        _t = move_and_append(_t);
        _t = move_and_append(_t);
        _t = move_and_append(_t);
        _t = move_and_append(_t);
        _t = move_and_append(_t);
        _t = move_and_append(_t);
        _t = move_and_append(_t);
        _t = move_and_append(_t);
        _t = move_and_append(_t);
        // println!("{t:?}");
    });
}

fn move_and_append(mut x: Vec<i32>) -> Vec<i32> {
    x.push(123);
    x.push(124);
    x.push(124);
    x.push(124);
    x.push(124);
    x.push(125);
    x.push(125);
    x.push(125);
    x.push(125);
    x.push(125);
    x.push(125);
    x
}

#[bench]
fn string_concat1(bench: &mut Bencher) {
    let s1 = String::from("this is a string");
    let s2 = String::from("smallstring1");
    let s3 = String::from("smallstring2");
    bench.iter(|| {
        let _s = s1.clone() + &s2 + &s3;
        // println!("{s}");
    });
}

#[bench]
fn string_concat2(bench: &mut Bencher) {
    let s1 = String::from("this is a string");
    let s2 = String::from("smallstring1");
    let s3 = String::from("smallstring2");
    bench.iter(|| {
        let _s: String = [&s1, &s2, &s3].map(|x| x.clone()).concat();
        // println!("{s}");
    });
}

#[bench]
fn string_concat3(bench: &mut Bencher) {
    let s = String::from("this is a string");
    let s2 = String::from("smallstring1");
    let s3 = String::from("smallstring2");
    bench.iter(
        || {
        let mut s = s.clone();
        s.reserve(s2.len() + s3.len());
        s.push_str(&s2);
        s.push_str(&s3);
        // println!("{s}");
    });
}

#[inline(never)]
fn t_black_box() {}

fn main() {
    // println!("hello, perf!");
}
