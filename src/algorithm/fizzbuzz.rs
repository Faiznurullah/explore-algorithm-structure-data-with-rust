pub fn get_fizzbuzz(n : i32) -> String{

     if n % 15 == 0 {
        return String::from("FizzBuzz");
     } else if n % 5 == 0 {
        return String::from("Buzz");
     } else if n % 3 == 0 {
        return String::from("Fizz");
     } else {
        return n.to_string();
     }

}

pub fn fizzbuzz_to(n: i32) {
    print!("");
    for i in 1..=n {
        println!("{}", get_fizzbuzz(i));
    }
}