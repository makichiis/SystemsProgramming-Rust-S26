
fn is_even(n: i32) -> bool {
    n % 2 == 0
}

fn get_parity_string(n: i32) -> String {
    if is_even(n) {
        return format!("{n} is even.");
    }
    format!("{n} is odd")
}

fn main() {
    let mut xs: [i32; 10] = [0; 10];
    xs[1] = 1;

    for x in xs {
        let div_3 = x % 3 == 0;
        let div_5 = x % 5 == 0;

        println!("{}", match (div_3, div_5) {
            (true, true) => String::from("FizzBuzz"),
            (true, false) => String::from("Fizz"),
            (false, true) => String::from("Buzz"),
            (false, false) => get_parity_string(x),
        });
    }

    let mut i: usize = 0;
    let mut sum: i32 = 0;
    while i < xs.len() { sum += xs[i]; i += 1; }

    let mut max: i32 = i32::MIN;
    for x in xs { if x > max { max = x; } }
    println!("Sum of array: {}\nMax of array: {}", sum, max);
}
