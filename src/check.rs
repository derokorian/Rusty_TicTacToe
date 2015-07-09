pub fn is_player_winner(board:&mut Vec<u8>, player: u8) -> bool {
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

pub fn is_board_full(board: &mut Vec<u8>) -> bool {
  for i in 0..9 {
    if board[i] == 0 {
      return false;
    }
  }
  return true;
}