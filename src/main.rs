fn get_value(num:i8) -> String {
	let mut result = String::from("");

	// match num {
	// 	num if num % 3 == 0 => {

	// 	},
	// }

	if num % 3 == 0 {
		result = "FIZZ".to_string();
	}

	if num % 5 == 0 {
		result += "BUZZ"
	}

	if result == "" {
		result = num.to_string();
	}

	return result
}

fn main() {
	for i in 1..26 {
		let value = get_value(i);
		println!("{}", value);
	}
}
