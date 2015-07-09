extern crate rand;

use std::io;
use self::rand::Rng;

pub fn get_user_move(board: &mut Vec<u8>) -> u32 {
	let mut umove = String::new();
	loop {
		println!("Enter your move (1-9):");
		io::stdin().read_line(&mut umove).ok().expect("Unable to read stdin");
		let mut umove: u32 = umove.trim().parse().ok().expect("Unable to parse input");

		if umove < 1 || umove > 9 {
			println!("Invalid move specified");
		} else {
			umove -= 1;
			if board[umove as usize] == 0 {
				return umove;
			} else {
				println!("Invalid move specified");
			}
		}
	}
}

pub fn get_cpu_move(mut board: &mut Vec<u8>) -> u32 {
	loop {
		let mut cmove = get_best_move(&mut board);
		if board[cmove as usize] == 0 {
			cmove += 1;
			println!("Opponent moves to {}", cmove);
			cmove -= 1;
			return cmove;
		}
	}
}

fn get_best_move(board: &mut Vec<u8>) -> u32 {
	let mut count_twos: u8 = 0;
	let mut count_ones: u8 = 0;
	let mut i = 0;
	loop {
		if board[i] == 2 {
			count_twos += 1;
		}
		if board[i] == 1 {
			count_ones += 1;
		}
		i += 1;
		if i == 9 {
			break;
		}
	}
	if count_twos < 2 {
		return rand::thread_rng().gen_range(0,9);
	}
	return rand::thread_rng().gen_range(0,9);
	// analyze the board look for open
}