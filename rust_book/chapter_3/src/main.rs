fn main() {
    println!("{}", fibo(12));
}

fn fizzbuzz(n: i32) {
    let mut i = 1;

    while i <= n {
        if (i % 3 == 0) && (i % 5 == 0) {
            println!("FizzBuzz");
        } else if i % 3 == 0 {
            println!("Fizz");
        } else if i % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{i}");
        }
        i = i + 1
    }
}

fn fibo(n: u64) -> u64 {
    let mut fn_1: u64 = 1;
    let mut fn_2: u64 = 1;
    let mut fn_n: u64;

    let mut i: u64 = 3;

    while i <= n {
        fn_n = fn_2 + fn_1;
        fn_1 = fn_2;
        fn_2 = fn_n;

        i = i + 1
    }
    fn_2
}