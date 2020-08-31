package main

import "fmt"

func duplicateZeros(arr []int) {

	for i := 0; i < len(arr); i++ {
		if arr[i] == 0 {
			if i < len(arr)-1 {
				copy(arr[i+2:], arr[i+1:len(arr)-1])
				arr[i+1] = 0
			}
			i++
		}
	}
}
func main() {
	fmt.Println("a")
}
