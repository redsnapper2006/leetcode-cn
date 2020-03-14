package main

import "fmt"

func getLeastNumbers(arr []int, k int) []int {
	if k >= len(arr) {
		return arr
	}

	var recur func(arr []int, k int)
	recur = func(arr []int, k int) {

		s, e := 0, len(arr)-1
		c := arr[s]
		for s < e {
			for s < e {
				if arr[s] > c {
					break
				}
				s++
			}

			for e > s {
				if arr[e] <= c {
					break
				}
				e--
			}
			if e > s {
				arr[s], arr[e] = arr[e], arr[s]
			}
		}
		if arr[e] < arr[0] {
			arr[0], arr[e] = arr[e], arr[0]
		}
		if e > k {
			recur(arr[0:e], k)
		} else if e < k {
			recur(arr[e:], k-e)
		} else {
			return
		}
	}
	recur(arr, k)
	return arr[0:k]
}

func main() {
	fmt.Println(getLeastNumbers([]int{3, 2, 1}, 2))
	fmt.Println(getLeastNumbers([]int{0, 1, 2, 1}, 3))
	fmt.Println(getLeastNumbers([]int{0, 1, 2, 3, 4, 0, 3, 3, 8, 1, 4, 6, 2, 8, 8, 15, 10, 0, 9, 9, 1, 2, 17, 8, 17, 25, 18, 18, 16, 13, 18, 29, 2, 3, 32, 2, 26, 23, 18, 8, 34, 8, 11, 36, 36, 39, 46, 30, 21, 25, 21, 14, 41, 10, 31, 55, 45, 16, 33, 47, 4, 52, 59, 60, 1, 43, 42, 10, 12, 56, 12, 27, 22, 52, 38, 12, 41, 42, 71, 5, 42, 76, 8, 3, 31, 65, 11, 29, 28, 68, 33, 50, 73, 87, 22, 68, 31, 1, 38, 89}, 60))

}
