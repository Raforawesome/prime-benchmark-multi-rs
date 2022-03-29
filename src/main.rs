use std::time::{ Instant };
use std::io;
use std::env;
use std::thread;
use std::thread::JoinHandle;

fn main() {
	let start = Instant::now();
	let max: i32 = 40000000;
	let mut thread_handles: Vec<JoinHandle<i32>> = Vec::new();
	let mut prime_count: i32 = 0;
	let threads = 16;

	for i in 1..=threads {
		let sub_sect: i32 = max / threads * i;
		let pre_sect: i32 = max / threads * (i - 1);
		thread_handles.push(thread::spawn(move || {
			let mut c: i32 = 0;
			for i in pre_sect..=sub_sect {
				if is_prime(i as f64) {
					c += 1;
				}
			}
			c
		}));
	}
	for handle in thread_handles {
		let res: i32 = handle.join().unwrap();
		prime_count += res;
	}

	println!("Found {} primes between {} and {}", prime_count, 1, max);
	let delta_time = start.elapsed();
	println!("Time elapsed: {:?}", delta_time);
	
	if env::consts::OS == "windows" {
		let mut tmp: String = "".to_string();
		println!("Press enter to exit program...");
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