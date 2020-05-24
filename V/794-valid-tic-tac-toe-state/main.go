package main

import "fmt"

func validTicTacToe(board []string) bool {
	buf := make([][]byte, 3)
	for i := 0; i < len(buf); i++ {
		buf[i] = make([]byte, 3)
	}

	OCnt, XCnt, SCnt := 0, 0, 0
	for i := 0; i < len(board); i++ {
		for j := 0; j < len(board[i]); j++ {
			buf[i][j] = board[i][j]
			if buf[i][j] == 'O' {
				OCnt++
			} else if buf[i][j] == 'X' {
				XCnt++
			} else {
				SCnt++
			}
		}
	}
	if XCnt-OCnt != 0 && XCnt-OCnt != 1 {
		return false
	}

	XWin, OWin := false, false
	for i := 0; i < 3; i++ {
		if (buf[i][0] != ' ' || buf[i][1] != ' ' || buf[i][2] != ' ') && buf[i][0] == buf[i][1] && buf[i][1] == buf[i][2] {
			if buf[i][0] == 'X' {
				XWin = true
			} else {
				OWin = true
			}
		}
		if (buf[0][i] != ' ' || buf[1][i] != ' ' || buf[2][i] != ' ') && buf[0][i] == buf[1][i] && buf[1][i] == buf[2][i] {
			if buf[0][i] == 'X' {
				XWin = true
			} else {
				OWin = true
			}
		}
	}

	if (buf[0][0] != ' ' || buf[1][1] != ' ' || buf[2][2] != ' ') && buf[0][0] == buf[1][1] && buf[1][1] == buf[2][2] {
		if buf[0][0] == 'X' {
			XWin = true
		} else {
			OWin = true
		}
	}
	if (buf[2][0] != ' ' || buf[1][1] != ' ' || buf[0][2] != ' ') && buf[2][0] == buf[1][1] && buf[1][1] == buf[0][2] {
		if buf[2][0] == 'X' {
			XWin = true
		} else {
			OWin = true
		}
	}

	if XWin && !OWin && (XCnt > OCnt) {
		return true
	}
	if OWin && !XWin && (XCnt == OCnt) {
		return true
	}
	if !XWin && !OWin {
		return true
	}
	return false
}

func main() {
	fmt.Println(validTicTacToe([]string{"XXX", "OOX", "OOX"}))

	fmt.Println(validTicTacToe([]string{"OXX", "XOX", "OXO"}))

}
