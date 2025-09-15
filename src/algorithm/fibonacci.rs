pub fn get_fibonacci(n: i32) -> i32 {
    if n <= 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }
    
    return get_fibonacci(n - 1) + get_fibonacci(n - 2);
}