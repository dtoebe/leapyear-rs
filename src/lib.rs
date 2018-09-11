#![feature(test)]
extern crate test;

pub fn gen_output_u32(year: u32) -> String {
    let f = |x| year % x == 0;
    let mut n = 0;
    if f(4) && (!f(100) || f(400)) {
        n = 1;
    }

    format!("{} - {}", year, n)
}

pub fn gen_output_u32_simple_n(year: u32) -> String {
    let f = |x| year % x == 0;
    let n = if f(4) && (!f(100) || f(400)) { 1 } else { 0 };

    format!("{} - {}", year, n)
}

pub fn gen_output_u32_no_format(year: u32) -> String {
    let f = |x| year % x == 0;
    let mut n = 0;
    if f(4) && (!f(100) || f(400)) {
        n = 1;
    }

    year.to_string() + " - " + &n.to_string()
}

pub fn gen_output_i32(year: i32) -> String {
    let f = |x| year % x == 0;
    let mut n = 0;
    if f(4) && (!f(100) || f(400)) {
        n = 1;
    }

    format!("{} - {}", year, n)
}

pub fn gen_output_usize(year: usize) -> String {
    let f = |x| year % x == 0;
    let mut n = 0;
    if f(4) && (!f(100) || f(400)) {
        n = 1;
    }

    format!("{} - {}", year, n)
}

pub fn gen_output_isize(year: isize) -> String {
    let f = |x| year % x == 0;
    let mut n = 0;
    if f(4) && (!f(100) || f(400)) {
        n = 1;
    }

    format!("{} - {}", year, n)
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_gen_output_u32(b: &mut Bencher) {
        b.iter(|| gen_output_u32(1900));
    }

    #[bench]
    fn bench_gen_output_u32_simple_n(b: &mut Bencher) {
        b.iter(|| gen_output_u32_simple_n(1900));
    }

    #[bench]
    fn bench_gen_output_u32_no_format(b: &mut Bencher) {
        b.iter(|| gen_output_u32_no_format(1900));
    }
    #[bench]
    fn bench_gen_output_i32(b: &mut Bencher) {
        b.iter(|| gen_output_i32(1900));
    }

    #[bench]
    fn bench_gen_output_usize(b: &mut Bencher) {
        b.iter(|| gen_output_usize(1900));
    }

    #[bench]
    fn bench_gen_output_isize(b: &mut Bencher) {
        b.iter(|| gen_output_isize(1900));
    }
}
