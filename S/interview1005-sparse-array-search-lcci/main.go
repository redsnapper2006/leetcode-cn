package main

import "fmt"

func findString(words []string, s string) int {
	var recur func(ws []string, ss string) int
	recur = func(ws []string, ss string) int {
		if len(ws) == 0 {
			return -1
		}
		s, e := 0, len(ws)-1
		m := s + (e-s)/2
		if ws[m] == ss {
			return m
		} else if ws[m] == "" {
			left := recur(ws[0:m], ss)
			if left != -1 {
				return left
			}
			right := recur(ws[m+1:], ss)
			if right != -1 {
				return m + 1 + right
			}
			return -1
		} else if ws[m] < ss {
			r := recur(ws[m+1:], ss)
			if r == -1 {
				return -1
			}
			return m + 1 + r
		} else {
			return recur(ws[0:m], ss)
		}
	}
	return recur(words, s)
}

func main() {
	fmt.Println("a")
}
