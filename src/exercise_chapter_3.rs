pub fn executer() {
    // let res = temp_convert(32.0, &String::from("fahrenheit"));
    // println!("Temperature: {res}");
    let res1 = fibonacci(49);
    println!("Fibonacci: {res1}");
    // let res2 = bleh(&String::from("Abhinavisabadcoder"));
    // println!("{res2}");
}

// Convert temperatures b/w Fahrenheit and Celsius
fn temp_convert(degrees: f64, to: &String) -> f64 {
    if to == "fahrenheit" {
        degrees * 9.0 / 5.0 + 32.0
    } else if to == "celsius" {
        (degrees - 32.0) * 5.0 / 9.0
    } else {
        -69.0
    }
}

// Generate nth Fibonacci number
fn fibonacci(n: u64) -> u64 {
    if n == 0 { return 0 };
    if n == 1 || n == 2 {
        return 1;
    }
    fibonacci(n-1) + fibonacci(n-2)
}

// bleh
fn bleh(words: &String) -> String {
    let temp = String::from(words);
    let split_words = temp.split(" ");
    for word in split_words {
        return String::from(word);
    }
    String::new()
}
