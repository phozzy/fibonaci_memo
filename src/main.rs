fn fibonaci(position: usize) -> u128 {
    let mut memo: Vec<u128> = vec![0, 1, 1];
    fib_memo(position, &mut memo)
}
fn fib_memo(position: usize, memo_l: &mut Vec<u128>) -> u128 {
    match memo_l.get(position) {
        Some(number) => *number,
        None => {
            let first: u128 = fib_memo(position - 1, memo_l);
            let second: u128 = fib_memo(position - 2, memo_l);
            memo_l.push(first + second);
            fib_memo(position, memo_l)
        },
    }
}
fn main() {
    println!("{}", fibonaci(2));
    println!("{}", fibonaci(3));
    println!("{}", fibonaci(50));
}
