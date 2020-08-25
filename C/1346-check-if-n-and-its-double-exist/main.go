package main

import "fmt"

func checkIfExist(arr []int) bool {
	M := map[int]int{}
	for i := 0; i < len(arr); i++ {
		M[arr[i]] = i
	}
	for i := 0; i < len(arr); i++ {
		v, ok := M[arr[i]*2]
		if ok && v != i {
			return true
		}
	}
	return false
}

func main() {
	fmt.Println("a")
}
