package main

import "fmt"

func numberOfLines(widths []int, S string) []int {
	c := 0
	l := 0
	for i := 0; i < len(S); i++ {
		w := widths[int(S[i]-'a')]
		if c+w > 100 {
			l++
			c = w
		} else {
			c += w
		}
	}
	return []int{l, c}
}

func main() {
	fmt.Println("a")
}
