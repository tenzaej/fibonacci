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

        if user_input.signum() == -1 {
            let calc: i64 = calc_fib(user_input.wrapping_neg());
            let answer: i64 = negafib_adjustment(user_input, calc);
            println!("{}", answer);
            break;
        } else {
            let answer: i64 = calc_fib(user_input);
            println!("{}", answer);
            break;
        }
    }
}

fn calc_fib(x: i64) -> i64 {
    let mut now: i64 = 0;
    let mut last_last: i64 = 0;
    let mut last: i64 = 1;

    if x == 0 {
        now = 0;
    } else if x == 1 {
        now = 1;
    } else {
        for _ in 2..(x + 1) {
            now = last + last_last;
            last_last = last;
            last = now;
        };
    }
    now
}

fn negafib_adjustment(player_choice: i64, x: i64) -> i64 {
    if player_choice % 2 == 0 { x.wrapping_neg() } else { x }
}
