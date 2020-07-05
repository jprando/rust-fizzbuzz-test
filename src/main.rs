fn get_value(num: i8) -> String {
	return match (num % 3, num % 5) {
		(0, 0) => "FIZZBUZZ".to_string(),
		(0, _) => "FIZZ".to_string(),
		(_, 0) => "BUZZ".to_string(),
		_ => num.to_string()
	};
}

fn main() {
	for i in 1..100 {
		let value = get_value(i);
		println!("{}", value);
	}
}
