package main

import "fmt"

func isValidSudoku(board [][]byte) bool {
	const N = 9
	for i := 0; i < N; i++ {
		b := make([]byte, N)
		for j := 0; j < N; j++ {
			if board[i][j] != '.' {
				if b[board[i][j]-'1'] == '1' {
					return false
				} else {
					b[board[i][j]-'1'] = '1'
				}
			}
		}
	}
	for i := 0; i < N; i++ {
		b := make([]byte, N)
		for j := 0; j < N; j++ {
			if board[j][i] != '.' {
				if b[board[j][i]-'1'] == '1' {
					return false
				} else {
					b[board[j][i]-'1'] = '1'
				}
			}
		}
	}
	for i := 0; i < N; i = i + 3 {
		for j := 0; j < N; j = j + 3 {
			b := make([]byte, N)
			for m := i; m < i+3; m++ {
				for n := j; n < j+3; n++ {
					if board[m][n] != '.' {
						if b[board[m][n]-'1'] == '1' {
							return false
						} else {
							b[board[m][n]-'1'] = '1'
						}
					}
				}
			}
		}
	}
	return true
}

func main() {
	fmt.Println("a")
}
