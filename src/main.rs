use std::io;

fn main() {
    let mut input: String = String::new();
	io::stdin().read_line(&mut input).unwrap();

	let mut nums = input.split_whitespace().map(|x| x.parse::<i32>().unwrap());
	let sum = nums.next().unwrap() + nums.next().unwrap();
	
    println!("{}", sum);
}
