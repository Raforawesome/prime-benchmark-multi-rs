use std::time::{ Instant };
use std::io;
use std::env;

fn main() {
	let start = Instant::now();
	for i in 1..=5000000 {
		if is_prime(i as f64) {
			println!("{} is a prime number", i);
		}
	}
	let delta_time = start.elapsed();
	println!("Time elapsed: {:?}", delta_time);


	if env::consts::OS == "windows" {
		let mut tmp: String = String::from("");
		println!("{}", "Press any key to exit the program...");
		let tmp2 = io::stdin().read_line(&mut tmp);

		std::mem::drop(tmp);
		std::mem::drop(tmp2);
	}
}

fn is_prime(num: f64) -> bool {
	if num == 2 as f64 {
		return true
	} else if num == 1 as f64 || num as i64 % 2 == 0 || num != (num as i64) as f64 {
		return false
	}

	let limit: i64 = num.sqrt() as i64;
	for i in 1..=limit {
		if i == 1 {
			continue
		} else {
			if num % (i as f64) == (0 as f64) {
				return false
			}
		}
	}
	return true
}