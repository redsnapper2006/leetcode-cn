package main

func sumSubarrayMins2(arr []int) int {
	M := 1000000007
	left, right := make([]int, len(arr)), make([]int, len(arr))
	for i := 0; i < len(arr); i++ {
		base := arr[i]
		cnt := 0
		for j := i; j < len(arr); j++ {
			if arr[j] >= base {
				cnt++
			} else {
				break
			}
		}
		left[i] = cnt
	}

	for i := len(arr) - 1; i >= 0; i-- {
		base := arr[i]
		cnt := 1
		for j := i - 1; j >= 0; j-- {
			if arr[j] > base {
				cnt++
			} else {
				break
			}
		}
		right[i] = cnt
	}
	// fmt.Println(left, right)

	sum := 0
	for i := 0; i < len(arr); i++ {
		sum += (left[i] * right[i] * arr[i])
		sum %= M
	}
	return sum
}
