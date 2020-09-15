package main

import "fmt"

func maxNumberOfBalloons(text string) int {
	buf := make([]int, 5)
	for _, b := range text {
		if b == 'b' {
			buf[0]++
		} else if b == 'a' {
			buf[1]++
		} else if b == 'l' {
			buf[2]++
		} else if b == 'o' {
			buf[3]++
		} else if b == 'n' {
			buf[4]++
		}
	}
	buf[2] /= 2
	buf[3] /= 2
	min := 1<<31 - 1
	for _, c := range buf {
		if min > c {
			min = c
		}
	}
	return min
}

func main() {
	fmt.Println("a")
}
