package main

import "fmt"

func commonChars(A []string) []string {
	buf := make([][]int, len(A))
	for i, w := range A {
		buf[i] = make([]int, 26)
		for _, c := range w {
			buf[i][c-'a']++
		}
	}
	var ret []string
	for i := 0; i < 26; i++ {
		min := 1<<31 - 1
		for _, b := range buf {
			if b[i] < min {
				min = b[i]
			}
		}
		ch := string(byte('a' + i))
		for m := 0; m < min; m++ {
			ret = append(ret, ch)
		}
	}
	return ret
}

func main() {
	fmt.Println("a")
}
