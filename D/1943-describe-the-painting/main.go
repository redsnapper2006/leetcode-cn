package main

import "fmt"

func splitPainting(segments [][]int) [][]int64 {
	min, max := 1000000, -1
	m := map[int]int{}
	for _, v := range segments {
		if min > v[0] {
			min = v[0]
		}
		if max < v[1] {
			max = v[1]
		}
		m[v[0]]++
		m[v[1]]++
	}

	buf := make([]int, max+1+1)
	for _, seg := range segments {
		buf[seg[0]] += seg[2]
		buf[seg[1]] -= seg[2]
	}
	for i := min + 1; i <= max+1; i++ {
		buf[i] += buf[i-1]
	}
	base := buf[min]
	start := min
	ret := [][]int64{}
	for i := min; i <= max+1; i++ {
		if i == min {
			continue
		}
		_, ok := m[i]
		if buf[i] == base && !ok {
			continue
		}

		if base != 0 {
			ret = append(ret, []int64{int64(start), int64(i), int64(base)})
		}
		start = i
		base = buf[i]
	}
	return ret
}

func main() {
	fmt.Println(splitPainting([][]int{{4, 16, 12}, {9, 10, 15}, {18, 19, 13}, {3, 13, 20}, {12, 16, 3}, {2, 10, 10}, {3, 11, 4}, {13, 16, 6}}))
}
