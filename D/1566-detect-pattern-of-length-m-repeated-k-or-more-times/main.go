package main

import "fmt"

func containsPattern(arr []int, m int, k int) bool {
	for i := 0; i <= len(arr)-m*k; i++ {
		isRepeat := true
		for j := 0; j < m; j++ {
			for n := 0; n < k; n++ {
				if arr[i+j] != arr[i+j+n*m] {
					isRepeat = false
					break
				}
			}
			if !isRepeat {
				break
			}
		}
		if isRepeat {
			return true
		}
	}
	return false
}

func main() {
	fmt.Println("a")
}
