package main

import "fmt"

func countBattleships(board [][]byte) int {
	cnt := 0
	for i := 0; i < len(board); i++ {
		for j := 0; j < len(board[0]); j++ {
			if board[i][j] == 'X' && (j == 0 || board[i][j-1] == '.') && (i == 0 || board[i-1][j] == '.') {
				cnt++
			}
		}
	}
	return cnt
}

func main() {
	fmt.Println()
}
