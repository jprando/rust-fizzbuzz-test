fn main() {
	(1..101u8).map(|num| match(num % 3, num % 5) {
		(0, 0) => "FIZZBUZZ".into(),
		(0, _) => "FIZZ".into(),
		(_, 0) => "BUZZ".into(),
		_ => num.to_string()
	}).for_each(|num| println!("{}", num))
}
