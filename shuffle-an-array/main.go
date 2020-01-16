package main

import (
	"fmt"
)

type Solution struct {
	Orgs    []int
	Channel chan []int
}

func Constructor(nums []int) Solution {
	channel := make(chan []int)

	if len(nums) > 0 {
		// Thanks -> Heap's algorithm to Generate permutations
		var helper func([]int, int)
		helper = func(arr []int, n int) {
			if n == 1 {
				tmp := make([]int, len(arr))
				copy(tmp, arr)
				channel <- tmp
			} else {
				for i := 0; i < n; i++ {
					helper(arr, n-1)
					if n%2 == 1 {
						tmp := arr[i]
						arr[i] = arr[n-1]
						arr[n-1] = tmp
					} else {
						tmp := arr[0]
						arr[0] = arr[n-1]
						arr[n-1] = tmp
					}
				}
				if len(arr) == n {
					helper(arr, n)
				}
			}
		}
		working := make([]int, len(nums))
		copy(working, nums)
		go helper(working, len(nums))
	}

	return Solution{Orgs: nums, Channel: channel}
}

/** Resets the array to its original configuration and return it. */
func (this *Solution) Reset() []int {
	return this.Orgs
}

/** Returns a random shuffling of the array. */
func (this *Solution) Shuffle() []int {
	if len(this.Orgs) > 0 {
		return <-this.Channel
	} else {
		return this.Orgs
	}
}

/**
 * Your Solution object will be instantiated and called as such:
 * obj := Constructor(nums);
 * param_1 := obj.Reset();
 * param_2 := obj.Shuffle();
 */
func main() {
	fmt.Println("a")
	// fmt.Println(Punc([]int{1, 2, 3, 4, 5}))
	// obj := Constructor([]int{-6, 10, 184})
	obj := Constructor([]int{0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93, 94, 95, 96, 97, 98, 99, -11, -99, -98, -97, -96, -95, -94, -93, -92, -91, -90, -89, -88, -87, -86, -85, -84, -83, -82, -81, -80, -79, -78, -77, -76, -75, -74, -73, -72, -71, -70, -69, -68, -67, -66, -65, -64, -63, -62, -61, -60, -59, -58, -57, -56, -55, -54, -53, -52, -51, -50, -49, -48, -47, -46, -45, -44, -43, -42, -41, -40, -39, -38, -37, -36, -35, -34, -33, -32, -31, -30, -29, -28, -27, -26, -25, -24, -23, -22, -21, -20, -19, -18, -17, -16, -15, -14, -13, -12, -2, -10, -9, -8, -7, -6, -5, -4, -3, -1})

	fmt.Println(obj.Reset())
	fmt.Println(obj.Shuffle())
	fmt.Println(obj.Shuffle())
	fmt.Println(obj.Shuffle())
	fmt.Println(obj.Shuffle())
	fmt.Println(obj.Shuffle())
	fmt.Println(obj.Shuffle())
	fmt.Println(obj.Shuffle())
	fmt.Println(obj.Shuffle())
	fmt.Println(obj.Shuffle())
	fmt.Println(obj.Shuffle())
	fmt.Println(obj.Shuffle())
	fmt.Println(obj.Shuffle())
	fmt.Println(obj.Shuffle())
	fmt.Println(obj.Shuffle())
}
