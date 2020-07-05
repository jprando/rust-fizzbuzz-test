fn get_value(num: i8) -> String {
	match num {
		num if num % 15 == 0 => return "FIZZBUZZ".to_string(),
		num if num % 5 == 0 => return "BUZZ".to_string(),
		num if num % 3 == 0 => {
			return "FIZZ".to_string();
		}
		_ => {
			return num.to_string();
		}
	}
}

fn main() {
	for i in 1..26 {
		let value = get_value(i);
		println!("{}", value);
		// println!("{}", get_value(i));
	}
}
