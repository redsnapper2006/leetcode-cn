package main

func longestEqualSubarray(nums []int, k int) int {
	pos := map[int][]int{}
	idx := map[int][]int{}

	ans := 1
	for i, v := range nums {
		pos[v] = append(pos[v], i)
		if _, ok := idx[v]; !ok {
			idx[v] = []int{0, 0}
		} else {
			val := idx[v]
			off := val[0]
			sum := val[1]
			size := len(pos[v])
			sum += pos[v][size-1] - pos[v][size-2] - 1
			for sum > k {
				sum -= pos[v][off+1] - pos[v][off] - 1
				off += 1
			}
			idx[v] = []int{off, sum}

			if size-off > ans {
				ans = size - off
			}
		}
	}

	return ans
}
