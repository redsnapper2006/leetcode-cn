package main

func numRookCaptures(board [][]byte) int {

	// find ROOK
	var r, c int
	for i := 0; i < len(board); i++ {
		for j := 0; j < len(board[0]); j++ {
			if board[i][j] == 'R' {
				r, c = i, j
			}
		}
	}

	s := 0
	// up
	for i := r - 1; i >= 0; i-- {
		if board[i][c] == 'B' {
			break
		}
		if board[i][c] == 'p' {
			s++
			break
		}
	}
	// left
	for j := c - 1; j >= 0; j-- {
		if board[r][j] == 'B' {
			break
		}
		if board[r][j] == 'p' {
			s++
			break
		}
	}
	// down
	for i := r + 1; i < len(board); i++ {
		if board[i][c] == 'B' {
			break
		}
		if board[i][c] == 'p' {
			s++
			break
		}
	}
	// right
	for j := c + 1; j < len(board[0]); j++ {
		if board[r][j] == 'B' {
			break
		}
		if board[r][j] == 'p' {
			s++
			break
		}
	}
	return s
}

func main() {
	fmt.Println("a")
}
