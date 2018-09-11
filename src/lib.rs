pub fn is_leapyear(year: u32) -> bool {
    let f = |x| year % x == 0;
    f(4) && (!f(100) || f(400))
}

pub fn gen_output(year: u32) -> String {
    let mut res: u32 = 0;
    if is_leapyear(year) {
        res = 1;
    }

    format!("{} - {}", year, res)
}
