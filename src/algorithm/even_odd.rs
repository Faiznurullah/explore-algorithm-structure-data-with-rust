
pub fn get_even_odd(n: i32) -> String {

    if n % 2 == 0 {
        return format!("{} is even", n);
    } else {
        return format!("{} is odd", n);
    }

}