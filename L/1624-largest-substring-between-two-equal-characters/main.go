package main

import "fmt"

func maxLengthBetweenEqualCharacters(s string) int {
	start, end := make([]int, 26), make([]int, 26)
	for i := 0; i < 26; i++ {
		start[i] = -1
	}
	for i, b := range s {
		idx := int(b - 'a')
		if start[idx] == -1 {
			start[idx] = i
			continue
		}
		end[idx] = i
	}
	max := -1
	for i := 0; i < 26; i++ {
		if start[i] != -1 && end[i]-start[i] > max {
			max = end[i] - start[i]
		}
	}
	return max - 1
}

func main() {
	fmt.Println("a")
}
