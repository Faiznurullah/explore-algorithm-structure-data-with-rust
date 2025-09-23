mod algorithm;
mod cryptography;

fn main() {
    // Fibonacci of 9
    let result = algorithm::fibonacci::get_fibonacci(9);
    println!("Fibonacci result: {}", result);

    // factorial of 5
    let fact_result = algorithm::factorial::factorial(5);
    println!("{}", fact_result);

    // even odd of 10
    let even_odd_result = algorithm::even_odd::get_even_odd(10);
    println!("{}", even_odd_result);

    // lcm of 4 and 5
    let lcm_result = algorithm::gcd_lcm::lcm(4, 5);
    println!("LCM of 4 and 5 is: {}", lcm_result);

    // gcd of 8 and 12
    let gcd_result = algorithm::gcd_lcm::gcd(8, 12);
    println!("GCD of 8 and 12 is: {}", gcd_result);

    // fizzbuzz to 15
    algorithm::fizzbuzz::fizzbuzz_to(15);

    // bubble sort
    let arr = vec![64, 34, 25, 12, 22, 11, 90];
    let sorted_arr = algorithm::sorting::bubble::bubble_sort(arr);
    println!("Sorted array: {:?}", sorted_arr); 

    // selection sort
    let arr2 = vec![64, 25, 12, 22, 11];
    let sorted_arr2 = algorithm::sorting::selection::selection_sort(arr2);
    println!("Sorted array: {:?}", sorted_arr2);

    // insertion sort
    let arr3 = vec![12, 11, 13, 5, 6];
    let sorted_arr3 = algorithm::sorting::insertion::insertion_sort(arr3);
    println!("Sorted array: {:?}", sorted_arr3);

    // linear search
    let arr4 = vec![10, 12, 11, 112, 56, 78, 22, 90];
    let target = 22;
    let search_result = algorithm::search::linear::linear_search(arr4, target);
    if search_result != -1 {
        println!("Element found at index: {}", search_result);
    } else {
        println!("Element not found in the array");
    }

    // binary search
    let arr5 = vec![10, 20, 30, 40, 50, 60, 70, 80, 90];
    let target2 = 70;
    let search_result2 = algorithm::search::binary::binary_search(arr5, target2);
    if search_result2 != -1 {
        println!("Element found at index: {}", search_result2);
    } else {
        println!("Element not found in the array");
    }


    // Stack 
    let mut stack = algorithm::stack::stack::Stack::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    stack.display();

    println!("Size: {}", stack.size());

    if let Some(top) = stack.peek() {
        println!("Top element: {}", top);
    }

    while !stack.is_empty() {
        if let Some(item) = stack.pop() {
            println!("Popped: {}", item);
        }
    }

    stack.display();


    // Queue
    let mut queue = algorithm::queue::queue::Queue::new();
    queue.enqueue(1); 
    queue.enqueue(2);
    queue.enqueue(3);
    queue.display();

    if let Some(front) = queue.front() {
        println!("Front element: {}", front);
    }

    if let Some(rear) = queue.rear() {
        println!("Rear element: {}", rear);
    }

    while !queue.is_empty() {
        if let Some(item) = queue.dequeue() {
            println!("Dequeued: {}", item);
        }
    }

    queue.display();


    // HashMap
    algorithm::hashmap::hashmap::try_hashmap();


    // Caesar Chiper
    let plaintext = "HELLO WORLD";
    let shift = 3;
    let ciphertext = cryptography::caesar::caesar_chiper(plaintext, shift);
    println!("Ciphertext: {}", ciphertext);

    // Affine Chiper
    let plaintext2 = "HELLO WORLD";
    let key = 5;
    let ciphertext2 = cryptography::affine::encrypt(plaintext2, key);
    println!("Affine Ciphertext: {}", ciphertext2);
    let decrypted_text = cryptography::affine::decrypt(&ciphertext2, key);
    println!("Decrypted Text: {}", decrypted_text);

    // Vigenere Chiper
    let plaintext3 = "HELLO WORLD";
    let key2 = "KEY";
    let ciphertext3 = cryptography::vigenere::encrypt(plaintext3, key2);
    println!("Vigenere Ciphertext: {}", ciphertext3);
    let decrypted_text2 = cryptography::vigenere::decrypt(&ciphertext3, key2);
    println!("Decrypted Text: {}", decrypted_text2);

    // vernam Chiper
    let plaintext4 = "HELLO WORLD";
    let key3 = "XMCKL ERTYU";
    let ciphertext4 = cryptography::vernam::encrypt(plaintext4, key3);
    println!("Vernam Ciphertext: {}", ciphertext4);
    let decrypted_text3 = cryptography::vernam::decrypt(&ciphertext4, key3);
    println!("Decrypted Text: {}", decrypted_text3);

    // Atbash Chiper
    let plaintext5 = "HELLO WORLD";
    let ciphertext5 = cryptography::atbash::encrypt(plaintext5);
    println!("Atbash Ciphertext: {}", ciphertext5);
    let decrypted_text4 = cryptography::atbash::decrypt(&ciphertext5);
    println!("Decrypted Text: {}", decrypted_text4);

    

}
