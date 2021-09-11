package main

import "fmt"

func flipChess(chessboard []string) int {
	board := make([][]int, len(chessboard))
	for i := 0; i < len(chessboard); i++ {
		board[i] = make([]int, len(chessboard[i]))
		for j, b := range chessboard[i] {
			if byte(b) == byte('X') {
				board[i][j] = 1
			} else if byte(b) == byte('O') {
				board[i][j] = 2
			}
		}
	}

	max := 0
	for i := 0; i < len(board); i++ {
		for j := 0; j < len(board[0]); j++ {
			if board[i][j] != 0 {
				continue
			}

			buf := make([][]int, len(board))
			for i := 0; i < len(board); i++ {
				buf[i] = make([]int, len(board[i]))
				copy(buf[i], board[i])
			}

			count := 0
			var dfs func(board [][]int, orgR, orgC int)
			dfs = func(board [][]int, orgR, orgC int) {
				board[orgR][orgC] = 1
				isTarget := []bool{false, false, false, false, false, false, false, false}
				cords := [][]int{{-1, -1}, {-1, 0}, {-1, 1}, {0, -1}, {0, 1}, {1, -1}, {1, 0}, {1, 1}}
				for cidx, cord := range cords {
					r, c := orgR+cord[0], orgC+cord[1]
					if !(r >= 0 && c >= 0 && r < len(board) && c < len(board[0]) && board[r][c] == 2) {
						continue
					}
					for r >= 0 && c >= 0 && r < len(board) && c < len(board[0]) {
						if board[r][c] == 2 {
							r += cord[0]
							c += cord[1]
						} else {
							break
						}
					}
					if r >= 0 && c >= 0 && r < len(board) && c < len(board[0]) && board[r][c] == 1 {
						isTarget[cidx] = true
					}
				}
				keep := false
				for _, t := range isTarget {
					keep = keep || t
				}
				if !keep {
					return
				}

				for cidx, cord := range cords {
					if !isTarget[cidx] {
						continue
					}

					r, c := orgR+cord[0], orgC+cord[1]
					for r >= 0 && c >= 0 && r < len(board) && c < len(board[0]) {
						if board[r][c] == 2 {
							count++
							dfs(board, r, c)
						} else {
							break
						}
					}
				}
			}
			dfs(buf, i, j)
			if count > max {
				max = count
			}
		}
	}
	return max
}

func main() {
	fmt.Println()
}
