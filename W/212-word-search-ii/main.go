package main

import (
	"fmt"
)

type Trie struct {
	M map[byte]*Trie
}

func findWords(board [][]byte, words []string) []string {
	trie := Trie{M: make(map[byte]*Trie)}

	for i := 0; i < len(words); i++ {
		p := &trie
		for j := 0; j < len(words[i]); j++ {
			_, ok := p.M[words[i][j]]
			if !ok {
				t := Trie{M: make(map[byte]*Trie)}
				p.M[words[i][j]] = &t
			}
			p = p.M[words[i][j]]
		}
	}

	buf := make([][]int, len(board))
	for i := 0; i < len(board); i++ {
		buf[i] = make([]int, len(board[0]))
	}

	M := make(map[string]int)
	for i := 0; i < len(words); i++ {
		M[words[i]] = 0
	}

	var dfs func(board [][]byte, buf [][]int, r, c int, T *Trie, M map[string]int, C []byte)
	dfs = func(board [][]byte, buf [][]int, r, c int, T *Trie, M map[string]int, C []byte) {
		// fmt.Println("entry", buf, r, c, C)

		v, ok := T.M[board[r][c]]
		if !ok {
			return
		}
		buf[r][c] = 1
		C = append(C, board[r][c])
		if _, ok2 := M[string(C)]; ok2 {
			M[string(C)]++
			// fmt.Println(string(C))
		}
		if r > 0 && buf[r-1][c] == 0 {
			dfs(board, buf, r-1, c, v, M, C)
		}
		if c > 0 && buf[r][c-1] == 0 {
			dfs(board, buf, r, c-1, v, M, C)
		}
		if r < len(board)-1 && buf[r+1][c] == 0 {
			dfs(board, buf, r+1, c, v, M, C)
		}
		if c < len(board[0])-1 && buf[r][c+1] == 0 {
			dfs(board, buf, r, c+1, v, M, C)
		}
		buf[r][c] = 0
	}

	for i := 0; i < len(board); i++ {
		for j := 0; j < len(board[0]); j++ {
			var t []byte
			dfs(board, buf, i, j, &trie, M, t)
		}
	}

	var ret []string
	for k, v := range M {
		if v > 0 {
			ret = append(ret, k)
		}
	}

	return ret
}

func main() {
	fmt.Println(findWords([][]byte{
		{'o', 'a', 'a', 'n'},
		{'e', 't', 'a', 'e'},
		{'i', 'h', 'k', 'r'},
		{'i', 'f', 'l', 'v'},
	}, []string{"oath", "pea", "eat", "rain"}))

	fmt.Println(findWords([][]byte{{'a'}}, []string{"a"}))
}
