fn main() {
    println!("Hello, world!");
    fizzBuzz();
}

fn fizzBuzz() {
    let mut count: u32 = 0;
    let mut fizzbuzzOccurence: u32 = 0;

    loop {
        count += 1;
        if count % 3 == 0 {
            println!("Fizz");
        }
        if count % 5 == 0 {
            println!("Buzz");
        }
        if count % 3 == 0 && count % 5 == 0 {
            println!("FizzBuzz");
            fizzbuzzOccurence += 1;
        }
        if count == 301 {
            break;
        }
    }
    println!("FizzBuzz occured {} times", fizzbuzzOccurence);
}
