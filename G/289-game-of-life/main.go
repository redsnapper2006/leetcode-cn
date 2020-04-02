package main

import "fmt"

func gameOfLife(board [][]int) {

	buf := make([]int, len(board[0]))
	for i := 0; i < len(board); i++ {
		t := make([]int, len(board[0]))
		for j := 0; j < len(board[0]); j++ {
			t[j] = board[i][j]
		}
		prev := 0
		for j := 0; j < len(board[0]); j++ {

			liveCount := 0
			if i > 0 {
				if j > 0 && buf[j-1] == 1 {
					liveCount++
				}
				if buf[j] == 1 {
					liveCount++
				}
				if j < len(board[0])-1 && buf[j+1] == 1 {
					liveCount++
				}
			}
			if j > 0 && prev == 1 {
				liveCount++
			}

			prev = board[i][j]

			if j < len(board[0])-1 && board[i][j+1] == 1 {
				liveCount++
			}
			if i < len(board)-1 {
				if j > 0 && board[i+1][j-1] == 1 {
					liveCount++
				}
				if board[i+1][j] == 1 {
					liveCount++
				}
				if j < len(board[0])-1 && board[i+1][j+1] == 1 {
					liveCount++
				}
			}

			if liveCount < 2 && board[i][j] == 1 {
				board[i][j] = 0
			} else if (liveCount == 2 || liveCount == 3) && board[i][j] == 1 {
				board[i][j] = 1
			} else if liveCount > 3 && board[i][j] == 1 {
				board[i][j] = 0
			} else if liveCount == 3 && board[i][j] == 0 {
				board[i][j] = 1
			}
		}
		buf = t
	}
}

func main() {

	a := [][]int{{0, 1, 0}, {0, 0, 1}, {1, 1, 1}, {0, 0, 0}}
	gameOfLife(a)
	fmt.Println(a)
}
