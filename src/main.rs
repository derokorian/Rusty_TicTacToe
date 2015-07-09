mod input;
mod check;

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

fn main() {
  let mut board: Vec<u8> = vec![0,0,0, 0,0,0, 0,0,0];
  let mut imove: u32;

  println!("1|2|3");
  println!("-----");
  println!("4|5|6");
  println!("-----");
  println!("7|8|9");

  loop {
    imove = input::get_user_move(&mut board);
    board[imove as usize] = 1;
    if check::is_player_winner(&mut board, 1) {
      print_board(&mut board, false);
      println!("You won!");
      break;
    } else if check::is_board_full(&mut board) {
      println!("Game over.");
      break;
    }

    imove = input::get_cpu_move(&mut board);
    board[imove as usize] = 2;
    print_board(&mut board, true);
    if check::is_player_winner(&mut board, 2) {
      println!("You lose!");
      break;
    } else if check::is_board_full(&mut board) {
      println!("Game over.");
      break;
    }
  }
}

