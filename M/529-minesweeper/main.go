package main

import "fmt"

func updateBoard(board [][]byte, click []int) [][]byte {
	if board[click[0]][click[1]] == 'M' {
		board[click[0]][click[1]] = 'X'
		return board
	}

	buf := [][]int{click}
	for len(buf) > 0 {
		var tt [][]int
		for i := 0; i < len(buf); i++ {
			if board[buf[i][0]][buf[i][1]] != 'E' {
				continue
			}
			var t [][]int
			mc := 0
			if buf[i][0] > 0 {
				if buf[i][1] > 0 {
					if board[buf[i][0]-1][buf[i][1]-1] == 'M' {
						mc++
					} else if board[buf[i][0]-1][buf[i][1]-1] == 'E' {
						t = append(t, []int{buf[i][0] - 1, buf[i][1] - 1})
					}
				}
				if board[buf[i][0]-1][buf[i][1]] == 'M' {
					mc++
				} else if board[buf[i][0]-1][buf[i][1]] == 'E' {
					t = append(t, []int{buf[i][0] - 1, buf[i][1]})
				}

				if buf[i][1] < len(board[0])-1 {
					if board[buf[i][0]-1][buf[i][1]+1] == 'M' {
						mc++
					} else if board[buf[i][0]-1][buf[i][1]+1] == 'E' {
						t = append(t, []int{buf[i][0] - 1, buf[i][1] + 1})
					}
				}
			}
			if buf[i][1] > 0 {
				if board[buf[i][0]][buf[i][1]-1] == 'M' {
					mc++
				} else if board[buf[i][0]][buf[i][1]-1] == 'E' {
					t = append(t, []int{buf[i][0], buf[i][1] - 1})
				}
			}
			if buf[i][1] < len(board[0])-1 {
				if board[buf[i][0]][buf[i][1]+1] == 'M' {
					mc++
				} else if board[buf[i][0]][buf[i][1]+1] == 'E' {
					t = append(t, []int{buf[i][0], buf[i][1] + 1})
				}
			}
			if buf[i][0] < len(board)-1 {
				if buf[i][1] > 0 {
					if board[buf[i][0]+1][buf[i][1]-1] == 'M' {
						mc++
					} else if board[buf[i][0]+1][buf[i][1]-1] == 'E' {
						t = append(t, []int{buf[i][0] + 1, buf[i][1] - 1})
					}
				}
				if board[buf[i][0]+1][buf[i][1]] == 'M' {
					mc++
				} else if board[buf[i][0]+1][buf[i][1]] == 'E' {
					t = append(t, []int{buf[i][0] + 1, buf[i][1]})
				}
				if buf[i][1] < len(board[0])-1 {
					if board[buf[i][0]+1][buf[i][1]+1] == 'M' {
						mc++
					} else if board[buf[i][0]+1][buf[i][1]+1] == 'E' {
						t = append(t, []int{buf[i][0] + 1, buf[i][1] + 1})
					}
				}
			}
			if mc == 0 {
				board[buf[i][0]][buf[i][1]] = 'B'
				tt = append(tt, t...)
			} else {
				board[buf[i][0]][buf[i][1]] = byte(mc + int('0'))
			}
		}
		buf = tt
	}
	return board
}

func main() {
	fmt.Println("a")
}
