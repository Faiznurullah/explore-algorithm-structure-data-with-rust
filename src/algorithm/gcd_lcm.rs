
pub fn gcd(mut a : i32, mut b : i32) -> i32 {
    
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }

    return a;
}

pub fn lcm(a : i32, b : i32) -> i32 {
    return (a * b) / gcd(a, b);
}