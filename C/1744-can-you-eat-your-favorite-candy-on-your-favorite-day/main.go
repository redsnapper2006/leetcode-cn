package main

import "fmt"

func canEat(candiesCount []int, queries [][]int) []bool {

	buf := make([]int, len(candiesCount))
	for i := 0; i < len(buf); i++ {
		if i == 0 {
			buf[i] = candiesCount[i]
		} else {
			buf[i] = buf[i-1] + candiesCount[i]
		}
	}

	ret := []bool{}

	for i := 0; i < len(queries); i++ {

		prevSum := 0
		if queries[i][0] > 0 {
			prevSum = buf[queries[i][0]-1]
		}
		if prevSum+1 > queries[i][2]*(queries[i][1]+1) {
			ret = append(ret, false)
			continue
		}
		if buf[queries[i][0]] >= queries[i][1]+1 {
			ret = append(ret, true)
		} else {
			ret = append(ret, false)
		}
	}
	return ret
}

func main() {
	fmt.Println(canEat([]int{46, 5, 47, 48, 43, 34, 15, 26, 11, 25, 41, 47, 15, 25, 16, 50, 32, 42, 32, 21, 36, 34, 50, 45, 46, 15, 46, 38, 50, 12, 3, 26, 26, 16, 23, 1, 4, 48, 47, 32, 47, 16, 33, 23, 38, 2, 19, 50, 6, 19, 29, 3, 27, 12, 6, 22, 33, 28, 7, 10, 12, 8, 13, 24, 21, 38, 43, 26, 35, 18, 34, 3, 14, 48, 50, 34, 38, 4, 50, 26, 5, 35, 11, 2, 35, 9, 11, 31, 36, 20, 21, 37, 18, 34, 34, 10, 21, 8, 5}, [][]int{{85, 54, 42}}))
}
