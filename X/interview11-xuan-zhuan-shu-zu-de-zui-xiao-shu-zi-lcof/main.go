package main

import "fmt"

func minArray(numbers []int) int {

	s, e := 0, len(numbers)-1
	for s < e {
		m := s + (e-s)/2
		if numbers[m] < numbers[e] {
			e = m
		} else if numbers[m] > numbers[e] {
			s = m + 1
		} else {
			e--
		}
	}
	return numbers[s]
}

func main() {
	fmt.Println("a")
}
