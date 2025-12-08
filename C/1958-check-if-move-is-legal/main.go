package main

import "fmt"

func checkMove(board [][]byte, rMove int, cMove int, color byte) bool {
	mid := byte('B')
	if color == 'B' {
		mid = byte('W')
	}

	if cMove >= 2 {
		idx := cMove - 1
		for idx >= 0 {
			if board[rMove][idx] != mid {
				break
			}
			idx--
		}
		if idx >= 0 && board[rMove][idx] == color && cMove-idx+1 >= 3 {
			return true
		}
	}
	if cMove+2 < len(board[0]) {
		idx := cMove + 1
		for idx < len(board[0]) {
			if board[rMove][idx] != mid {
				break
			}
			idx++
		}
		if idx < len(board[0]) && board[rMove][idx] == color && idx-cMove+1 >= 3 {
			return true
		}
	}
	if rMove >= 2 {
		idx := rMove - 1
		for idx >= 0 {
			if board[idx][cMove] != mid {
				break
			}
			idx--
		}
		if idx >= 0 && board[idx][cMove] == color && rMove-idx+1 >= 3 {
			return true
		}
	}
	if rMove+2 < len(board) {
		idx := rMove + 1
		for idx < len(board) {
			if board[idx][cMove] != mid {
				break
			}
			idx++
		}
		if idx < len(board) && board[idx][cMove] == color && idx-rMove+1 >= 3 {
			return true
		}
	}
	if rMove >= 2 && cMove >= 2 {
		idx1 := rMove - 1
		idx2 := cMove - 1
		for idx1 >= 0 && idx2 >= 0 {
			if board[idx1][idx2] != mid {
				break
			}
			idx1--
			idx2--
		}
		if idx1 >= 0 && idx2 >= 0 && board[idx1][idx2] == color && rMove-idx1+1 >= 3 {
			return true
		}
	}
	if rMove >= 2 && cMove+2 < len(board[0]) {
		idx1 := rMove - 1
		idx2 := cMove + 1
		for idx1 >= 0 && idx2 < len(board[0]) {
			if board[idx1][idx2] != mid {
				break
			}
			idx1--
			idx2++
		}
		if idx1 >= 0 && idx2 < len(board[0]) && board[idx1][idx2] == color && rMove-idx1+1 >= 3 {
			return true
		}
	}
	if rMove+2 < len(board) && cMove >= 2 {
		idx1 := rMove + 1
		idx2 := cMove - 1
		for idx1 < len(board) && idx2 >= 0 {
			if board[idx1][idx2] != mid {
				break
			}
			idx1++
			idx2--
		}
		if idx1 < len(board) && idx2 >= 0 && board[idx1][idx2] == color && idx1-rMove+1 >= 3 {
			return true
		}
	}
	if rMove+2 < len(board) && cMove+2 < len(board[0]) {
		idx1 := rMove + 1
		idx2 := cMove + 1
		for idx1 < len(board) && idx2 < len(board[0]) {
			if board[idx1][idx2] != mid {
				break
			}
			idx1++
			idx2++
		}
		if idx1 < len(board) && idx2 < len(board[0]) && board[idx1][idx2] == color && idx1-rMove+1 >= 3 {
			return true
		}
	}
	return false
}

func main() {
	fmt.Println()
}
