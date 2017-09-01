fn main() {
    let x: i32 = 7;
    let mut now: i32 = 0;
    let mut last_last: i32 = 0;
    let mut last: i32 = 1;

    for elem in 2..(x + 1) {
        now = last + last_last;
        println!("{}: {} = {} + {}", elem, now, last, last_last);
        last_last = last;
        last = now;
    }
}
