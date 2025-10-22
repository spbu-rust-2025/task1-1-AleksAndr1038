use std::io;

fn main() {
    let mut input: String = String::new();
	io::stdin().read_line(&mut input).unwrap();

	let mut scanner = input.split_whitespace();
	let a: i32 = scanner.next().unwrap().parse().unwrap();
	let b: i32 = scanner.next().unwrap().parse().unwrap();
	
    println!("{}", a + b);
}
