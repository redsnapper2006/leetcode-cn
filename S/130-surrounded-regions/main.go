package main

import "fmt"

func solve(board [][]byte) {
	if len(board) == 0 || len(board[0]) == 0 {
		return
	}
	buf := make([][]int, len(board))
	for i := 0; i < len(buf); i++ {
		buf[i] = make([]int, len(board[0]))
	}

	var work [][]int
	for i := 0; i < len(board[0]); i++ {
		if board[0][i] == 'O' {
			buf[0][i] = 1
			work = append(work, []int{0, i})
		}
		if board[len(board)-1][i] == 'O' {
			buf[len(board)-1][i] = 1
			work = append(work, []int{len(board) - 1, i})
		}
	}

	for i := 0; i < len(board); i++ {
		if board[i][0] == 'O' {
			buf[i][0] = 1
			work = append(work, []int{i, 0})
		}
		if board[i][len(board[0])-1] == 'O' {
			buf[i][len(board[0])-1] = 1
			work = append(work, []int{i, len(board[0]) - 1})
		}
	}

	for len(work) > 0 {
		var t [][]int
		for i := 0; i < len(work); i++ {
			r, c := work[i][0], work[i][1]
			if r > 0 && board[r-1][c] == 'O' && buf[r-1][c] == 0 {
				buf[r-1][c] = 1
				t = append(t, []int{r - 1, c})
			}
			if c > 0 && board[r][c-1] == 'O' && buf[r][c-1] == 0 {
				buf[r][c-1] = 1
				t = append(t, []int{r, c - 1})
			}
			if r < len(board)-1 && board[r+1][c] == 'O' && buf[r+1][c] == 0 {
				buf[r+1][c] = 1
				t = append(t, []int{r + 1, c})
			}
			if c < len(board[0])-1 && board[r][c+1] == 'O' && buf[r][c+1] == 0 {
				buf[r][c+1] = 1
				t = append(t, []int{r, c + 1})
			}
		}
		work = t
	}

	for i := 0; i < len(board); i++ {
		for j := 0; j < len(board[0]); j++ {
			if board[i][j] == 'O' && buf[i][j] == 0 {
				board[i][j] = 'X'
			}
		}
	}
}

func main() {
	b := [][]byte{
		[]byte{'X', 'X', 'X', 'X'},
		[]byte{'X', 'O', 'O', 'X'},
		[]byte{'X', 'X', 'O', 'X'},
		[]byte{'X', 'O', 'X', 'X'},
	}
	solve(b)
	fmt.Println(b)
}
