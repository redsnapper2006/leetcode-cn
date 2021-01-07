package main

import "fmt"

func tictactoe(board []string) string {
	isPending := false
	for i := 0; i < len(board); i++ {
		for j := 0; j < len(board[i]); j++ {
			if board[i][j] == ' ' {
				isPending = true
			}
		}
	}

	isWin := false
	winner := byte(' ')
	for i := 0; i < len(board); i++ {
		if board[i][0] != ' ' {
			isSame := true
			for j := 1; j < len(board[i]); j++ {
				if board[i][0] != board[i][j] {
					isSame = false
					break
				}
			}
			if isSame {
				isWin = true
				winner = board[i][0]
			}
		}
		if board[0][i] != ' ' {
			isSame := true
			for j := 1; j < len(board); j++ {
				if board[0][i] != board[j][i] {
					isSame = false
					break
				}
			}
			if isSame {
				isWin = true
				winner = board[0][i]
			}
		}
	}

	if board[0][0] != ' ' {
		isSame := true
		for i := 1; i < len(board); i++ {
			if board[0][0] != board[i][i] {
				isSame = false
				break
			}
		}
		if isSame {
			isWin = true
			winner = board[0][0]
		}
	}

	if board[len(board)-1][0] != ' ' {
		isSame := true
		for i := 1; i < len(board); i++ {
			if board[len(board)-1][0] != board[len(board)-1-i][i] {
				isSame = false
				break
			}
		}
		if isSame {
			isWin = true
			winner = board[len(board)-1][0]
		}
	}

	if isWin {
		return string(winner)
	}
	if isPending {
		return "Pending"
	}
	return "Draw"
}

func main() {
	fmt.Println()
}
