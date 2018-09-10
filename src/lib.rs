pub fn is_leapyear(year: u32) -> bool {
    let f = |x| year % x == 0;
    f(4) && (!f(100) || f(400))
}
