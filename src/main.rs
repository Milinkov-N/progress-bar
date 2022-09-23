#![allow(dead_code)]

use std::{thread::sleep, time::Duration};

const CLEAR: &str = "\x1b[2J\x1b[1;1H";

struct Unbounded;

struct Bounded {
    bound: usize,
    delims: (char, char),
}

struct Progress<Iter, Bound> {
    iter: Iter,
    i: usize,
    bound: Bound,
}

impl<Iter> Progress<Iter, Unbounded> {
    pub fn new(iter: Iter) -> Self {
        Self {
            iter,
            i: 0,
            bound: Unbounded,
        }
    }
}

impl<Iter> Progress<Iter, Bounded> {
    pub fn with_delims(mut self, delims: (char, char)) -> Self {
        self.bound.delims = delims;
        self
    }
}

impl<Iter, Bound> Iterator for Progress<Iter, Bound>
where
    Iter: Iterator,
    Bound: ProgressDisplay,
{
    type Item = Iter::Item;

    fn next(&mut self) -> Option<Self::Item> {
        print!("{CLEAR}");
        self.bound.display(&self);

        self.i += 1;
        self.iter.next()
    }
}

impl<Iter> Progress<Iter, Unbounded>
where
    Iter: ExactSizeIterator,
{
    pub fn with_bound(self) -> Progress<Iter, Bounded> {
        let bound = Bounded {
            bound: self.iter.len(),
            delims: ('[', ']'),
        };

        Progress {
            iter: self.iter,
            i: self.i,
            bound,
        }
    }
}

trait ProgressDisplay: Sized {
    fn display<Iter>(&self, progress: &Progress<Iter, Self>);
}

impl ProgressDisplay for Unbounded {
    fn display<Iter>(&self, progress: &Progress<Iter, Self>) {
        println!("{}", "*".repeat(progress.i))
    }
}

impl ProgressDisplay for Bounded {
    fn display<Iter>(&self, progress: &Progress<Iter, Self>) {
        println!(
            "{}{}{}{}",
            self.delims.0,
            "*".repeat(progress.i),
            " ".repeat(progress.bound.bound - progress.i),
            self.delims.1
        )
    }
}

fn progress<Iter>(iter: Iter, f: fn(Iter::Item) -> ())
where
    Iter: Iterator,
{
    let mut i = 1;

    for n in iter {
        println!("{}{}", CLEAR, "*".repeat(i));
        f(n);
        i += 1;
    }
}

trait ProgressIteratorExt: Sized {
    fn progress(self) -> Progress<Self, Unbounded>;
}

impl<Iter> ProgressIteratorExt for Iter
where
    Iter: Iterator,
{
    fn progress(self) -> Progress<Self, Unbounded> {
        Progress::new(self)
    }
}

fn expensive_calculation(_n: &i32) {
    sleep(Duration::from_secs(1));
}

fn main() {
    let brackets = ('<', '>');
    let vec = vec![1, 2, 3];
    let unbounded_range = (1..).progress();

    vec.iter()
        .progress()
        .with_bound()
        .with_delims(brackets)
        .for_each(|n| expensive_calculation(n));

    for n in unbounded_range {
        expensive_calculation(&n);
    }
}
