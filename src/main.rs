use std::io;

fn main() {
    loop {
        println!("Which Fibonacci number would you like?");
        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input)
            .expect("failed to read line;");
        let user_input: i64 = match user_input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let answer = neg_or_pos_fib(user_input);
        println!("number {} in the Fibonacci sequence is {}", user_input, answer);
        break;
    }
}

fn calc_fibonacci_from_position(x: i64) -> i64 {
    match x {
        0 => 0,
        1 => 1,
        _ => calc_fibonacci_actually(x)
    }
}

fn calc_fibonacci_actually(x: i64) -> i64 {
    let mut last_last: i64 = 0;
    let mut last: i64 = 1;
    let mut now: i64 = 0;
    for _ in 2..(x + 1) {
        now = last + last_last;
        last_last = last;
        last = now
    }
    now
}

fn neg_or_pos_fib(user_input: i64) -> i64 {
    if user_input.signum() == -1 {
        let calc: i64 = calc_fibonacci_from_position(user_input.wrapping_neg());
        negafib_adjustment(user_input, calc)
    } else {
        calc_fibonacci_from_position(user_input)
    }
}

fn negafib_adjustment(player_choice: i64, x: i64) -> i64 {
    if player_choice % 2 == 0 { x.wrapping_neg() } else { x }
}
