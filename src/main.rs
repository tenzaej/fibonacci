use std::io; // bring the `io` library into scope

fn main() {

    println!("Which Fibonacci number would you like?");

    loop {
        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input)
            .expect("failed to read line;"); // one line of code, two lines of text;
        let user_input: i64 = match user_input.trim().parse() {
            Ok(num) => num,
            Err(_) => -1,
        };

        if user_input.signum() == -1 {
            println!{"You must input a non-negative integer; please try again."};
            continue;
        } else {
            let mut now: u64 = 0;
            let mut last_last: u64 = 0;
            let mut last: u64 = 1;

            if user_input == 0 {
                0
            } else if user_input == 1 {
                1
            } else {
                for _ in 2..(user_input + 1) {
                    now = last + last_last;
                    // println!("{}: {} = {} + {}", elem, now, last, last_last);
                    last_last = last;
                    last = now;
                };
                now
            };
            println!("Fibonacci #{} is {}", user_input, now);
            break;
        }
    }
}
