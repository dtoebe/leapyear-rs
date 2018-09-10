use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 || args[1].len() < 4 {
        println!("please input a year YYYY");
        process::exit(1);
    }

    let year = match args[1].parse::<u32>() {
        Ok(n) => n,
        Err(_) => {
            println!("year is not a number..");
            process::exit(1);
        }
    };

    let mut res: u32 = 0;
    if is_leapyear(year) {
        res = 1;
    }

    println!("{} - {}", year, res);
}

fn is_leapyear(year: u32) -> bool {
    let f = |x| year % x == 0;
    f(4) && (!f(100) || f(400))
}
