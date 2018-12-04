fn main() {
    let list = 1..31;
    for x in list {
        println!("{}", fizzbuzz(x));
    }
}

fn fizzbuzz(num: i32) -> &'static str {
    let str_num: &str = &num.to_string();

    match num {
        num if num % 3 == 0 => return "Fizz",
        num if num % 5 == 0 => return "Buzz",
        num if num % 15 == 0 => return "FizzBuzz",
        _ => return str_num
    }
}
