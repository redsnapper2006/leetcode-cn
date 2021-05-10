package main

import "fmt"

func maximumPopulation(logs [][]int) int {
	buf := make([]int, 101)
	for i := 0; i < len(logs); i++ {
		buf[logs[i][0]-1950]++
		buf[logs[i][1]-1950]--
	}

	max := buf[0]
	year := 0
	for i := 1; i < len(buf); i++ {
		buf[i] += buf[i-1]
		if buf[i] > max {
			max = buf[i]
			year = i
		}
	}
	return year + 1950
}

func main() {
	fmt.Println()
}
