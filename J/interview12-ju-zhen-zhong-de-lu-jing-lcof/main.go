package main

import "fmt"

func exist(board [][]byte, word string) bool {

	b := make([][]int, len(board))
	for i := 0; i < len(board); i++ {
		b[i] = make([]int, len(board[0]))
	}

	var stack [][][]int
	var t [][]int
	for i := 0; i < len(board); i++ {
		for j := 0; j < len(board[0]); j++ {
			if board[i][j] == word[0] {
				t = append(t, []int{i, j})
			}
		}
	}
	stack = append(stack, t)
	var push [][]int
	for len(stack) > 0 {
		l := stack[len(stack)-1]
		if len(l) == 0 {
			stack = stack[0 : len(stack)-1]
			if len(stack) == 0 {
				continue
			}
			p := push[len(push)-1]
			b[p[0]][p[1]] = 0
			push = push[0 : len(push)-1]
			continue
		}
		base := l[0]
		stack[len(stack)-1] = l[1:]
		r, c := base[0], base[1]
		push = append(push, []int{r, c})
		b[r][c] = 1
		offset := len(stack)
		if offset == len(word) {
			return true
		}
		var n [][]int
		if r > 0 && b[r-1][c] != 1 && board[r-1][c] == word[offset] {
			n = append(n, []int{r - 1, c})
		}
		if c > 0 && b[r][c-1] != 1 && board[r][c-1] == word[offset] {
			n = append(n, []int{r, c - 1})
		}
		if r < len(board)-1 && b[r+1][c] != 1 && board[r+1][c] == word[offset] {
			n = append(n, []int{r + 1, c})
		}
		if c < len(board[0])-1 && b[r][c+1] != 1 && board[r][c+1] == word[offset] {
			n = append(n, []int{r, c + 1})
		}
		stack = append(stack, n)
	}
	return false
}

func main() {
	fmt.Println("a")
}
