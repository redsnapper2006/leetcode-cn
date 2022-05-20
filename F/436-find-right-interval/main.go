package main

func findRightInterval(intervals [][]int) []int {
	min, max := intervals[0][0], intervals[0][1]
	for _, v := range intervals {
		if v[0] < min {
			min = v[0]
		}
		if v[1] > max {
			max = v[1]
		}
	}

	buf := make([]int, max-min+1)
	for i := 0; i < max-min+1; i++ {
		buf[i] = -1
	}
	for i, v := range intervals {
		buf[v[0]-min] = i
	}
	prev := -1
	for i := max - min; i >= 0; i-- {
		if buf[i] != -1 {
			prev = buf[i]
		} else {
			buf[i] = prev
		}
	}

	ret := []int{}
	for _, v := range intervals {
		ret = append(ret, buf[v[1]-min])
	}
	return ret
}
