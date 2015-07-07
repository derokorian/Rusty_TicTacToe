extern crate rand;

use std::io;
use rand::Rng;

fn print_board(board: &mut Vec<u8>, show_moves: bool) {
  let mut i = 0;
  let mut line = String::new();
  loop {
    if board[i] == 1 {
      line.push_str("X");
    } else if board[i] == 2 {
      line.push_str("O");
    } else {
      line.push_str(" ");
    }
    i += 1;
    if i%3 == 0 {
      if show_moves {
        if i == 3 {
          line.push_str("\t\t1|2|3");
        } else if i == 6 {
          line.push_str("\t\t4|5|6");
        } else {
          line.push_str("\t\t7|8|9");
        }
      }
      println!("{}", line);
      if i < 9 {
        if show_moves {
          println!("-----\t\t-----");
        } else {
          println!("-----");
        }
      }
      line = String::new();
    } else {
      line.push_str("|");
    }
    if i == 9 {
      break;
    }
  }
}

fn get_user_move(mut board: &mut Vec<u8>) {
  let mut umove = String::new();
  loop {
    println!("Enter your move (1-9):");
    io::stdin().read_line(&mut umove)
        .ok().expect("Failed to read move");
    let mut umove: u32 = umove.trim().parse()
        .ok().expect("Failed to parse move as number");
    if umove < 1 || umove > 9 {
      println!("Invalid move specified");
    } else {
      umove -= 1;
      if board[umove as usize] == 0 {
        board[umove as usize] = 1;
        break;
      } else {
        println!("Invalid move specified");
      }
    }
  }
}

fn get_cpu_move(mut board:&mut Vec<u8>) {
  loop {
    let mut cmove = rand::thread_rng().gen_range(0,9);
    if board[cmove as usize] == 0 {
      board[cmove as usize] = 2;
      cmove += 1;
      println!("Opponent moves to {}", cmove);
      break;
    }
  }
  print_board(&mut board, true);
}

fn is_player_winner(board:&mut Vec<u8>, player: u8) -> bool {
  if board[0] == player {
    if board[1] == player {
      if board[2] == player {
        return true;
      }
    }
    if board[3] == player {
      if board[6] == player {
        return true;
      }
    }
    if board[4] == player {
      if board[8] == player {
        return true;
      }
    }
  }
  if board[1] == player {
    if board[4] == player {
      if board[7] == player {
        return true;
      }
    }
  }
  if board[2] == player {
    if board[5] == player {
      if board[8] == player {
        return true;
      }
    }
    if board[4] == player {
      if board[6] == player {
        return true;
      }
    }
  }
  if board[3] == player {
    if board[4] == player {
      if board[5] == player {
        return true;
      }
    }
  }
  if board[6] == player {
    if board[7] == player {
      if board[8] == player {
        return true;
      }
    }
  }
  return false;
}

fn is_board_full(board: &mut Vec<u8>) -> bool {
  for i in 0..9 {
    if board[i] == 0 {
      return false;
    }
  }
  return true;
}

fn main() {
  let mut board: Vec<u8> = vec![0,0,0, 0,0,0, 0,0,0];

  println!("1|2|3");
  println!("-----");
  println!("4|5|6");
  println!("-----");
  println!("7|8|9");

  loop {
    get_user_move(&mut board);
    if is_player_winner(&mut board, 1) {
      print_board(&mut board, false);
      println!("You won!");
      break;
    } else if is_board_full(&mut board) {
      println!("Game over.");
      break;
    }

    get_cpu_move(&mut board);
    if is_player_winner(&mut board, 2) {
      println!("You lose!");
      break;
    } else if is_board_full(&mut board) {
      println!("Game over.");
      break;
    }
  }
}

