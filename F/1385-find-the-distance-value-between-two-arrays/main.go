package main

import "fmt"

func findTheDistanceValue(arr1 []int, arr2 []int, d int) int {

	c := 0
	for _, v := range arr1 {
		isValid := true
		for _, b := range arr2 {
			dd := v - b
			if dd >= -d && dd <= d {
				isValid = false
				break
			}
		}
		if isValid {
			c++
		}
	}
	return c
}

func main() {
	fmt.Println("a")
}
