package main

import "fmt"

func findLucky(arr []int) int {
	buf := map[int]int{}
	for i := 0; i < len(arr); i++ {
		buf[arr[i]]++
	}
	ret := -1
	for k, v := range buf {
		if k == v && k > ret {
			ret = k
		}
	}
	return ret
}

func main() {
	fmt.Println("a")
}
