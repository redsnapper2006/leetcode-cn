package main

import "fmt"

func findSwapValues(array1 []int, array2 []int) []int {
	sum1, sum2 := 0, 0
	buf1, buf2 := map[int]int{}, map[int]int{}
	for i := 0; i < len(array1); i++ {
		sum1 += array1[i]
		buf1[array1[i]]++
	}
	for i := 0; i < len(array2); i++ {
		sum2 += array2[i]
		buf2[array2[i]]++
	}

	extract := sum1 - sum2
	if extract%2 == 1 || extract%2 == -1 {
		return []int{}
	}
	for i := 0; i < len(array2); i++ {
		_, ok := buf1[array2[i]+extract/2]
		if ok {
			return []int{array2[i] + extract/2, array2[i]}
		}
	}

	return []int{}
}

func main() {
	fmt.Println("a")
}
