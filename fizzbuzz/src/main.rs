fn main() {
    let list = 1..31;
    for x in list {
        println!("{}", fizzbuzz(x));
    }
}

fn fizzbuzz(num: i32) -> String {
    match num {
        num if num % 3 == 0 => return "Fizz".to_string(),
        num if num % 5 == 0 => return "Buzz".to_string(),
        num if num % 15 == 0 => return "FizzBuzz".to_string(),
        _ => return num.to_string()
    }
}
