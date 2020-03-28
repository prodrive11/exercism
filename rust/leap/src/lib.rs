pub fn is_leap_year(year: u64) -> bool {
    [4, 100, 400].iter().filter(|i| (year % *i) == 0).count() % 2 > 0
}
