package main

import (
	"fmt"
)

func tictactoe(moves [][]int) string {
	board := [][]int{{0, 0, 0}, {0, 0, 0}, {0, 0, 0}}
	for i, m := range moves {
		c := 0
		if i%2 == 0 {
			c = 1
		} else {
			c = 2
		}
		board[m[0]][m[1]] = c
	}

	win := 0
	for i := 0; i < 3; i++ {
		if board[i][0] == board[i][1] && board[i][1] == board[i][2] && board[i][0] != 0 {
			win = board[i][0]
			break
		}
	}
	if win > 0 {
		if win == 1 {
			return "A"
		} else {
			return "B"
		}
	}
	win = 0
	for i := 0; i < 3; i++ {
		if board[0][i] == board[1][i] && board[1][i] == board[2][i] && board[0][i] != 0 {
			win = board[0][i]
			break
		}
	}
	if win > 0 {
		if win == 1 {
			return "A"
		} else {
			return "B"
		}
	}
	if board[2][0] == board[1][1] && board[1][1] == board[0][2] && board[0][2] != 0 {
		if board[2][0] == 1 {
			return "A"
		} else {
			return "B"
		}
	}
	if board[2][2] == board[1][1] && board[1][1] == board[0][0] && board[0][0] != 0 {
		if board[2][2] == 1 {
			return "A"
		} else {
			return "B"
		}
	}
	if len(moves) < 9 {
		return "Pending"
	} else {
		return "Draw"
	}

}

func main() {
	// fmt.Println(tictactoe([][]int{{0, 0}, {2, 0}, {1, 1}, {2, 1}, {2, 2}}))
	fmt.Println(tictactoe([][]int{{0, 2}, {0, 1}, {2, 1}}))

}
