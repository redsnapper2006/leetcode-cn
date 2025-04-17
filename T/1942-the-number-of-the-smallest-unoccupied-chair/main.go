package main

import "fmt"

func smallestChair(times [][]int, targetFriend int) int {
	max := -1
	for _, t := range times {
		if max < t[1] {
			max = t[1]
		}
	}

	arrival, leave := make([][]int, max+1), make([][]int, max+1)
	for i, t := range times {
		if len(arrival[t[0]]) == 0 {
			arrival[t[0]] = []int{}
		}
		arrival[t[0]] = append(arrival[t[0]], i)
		if len(leave[t[1]]) == 0 {
			leave[t[1]] = []int{}
		}
		leave[t[1]] = append(leave[t[1]], i)
	}
	// fmt.Println("arrival ", arrival, "leave", leave)

	buf := []int{}
	for i := 0; i < max+1; i++ {
		if len(leave[i]) > 0 {
			for _, c := range leave[i] {
				idx := -1
				for j, b := range buf {
					if b == c {
						idx = j
						break
					}
				}
				buf[idx] = -1
			}
		}
		for _, c := range arrival[i] {
			idx := -1
			for j, b := range buf {
				if b == -1 {
					idx = j
					break
				}
			}
			if idx != -1 {
				buf[idx] = c
			} else {
				buf = append(buf, c)
			}
		}
		// fmt.Println(buf)
		if i == times[targetFriend][0] {
			for j, v := range buf {
				if v == targetFriend {
					return j
				}
			}
		}
	}
	return -1
}

func main() {
	fmt.Println(smallestChair([][]int{{3, 100000}, {37, 100000}, {34, 100000}, {16, 100000}, {28, 100000}, {19, 100000}, {14, 100000}, {29, 100000}, {5, 100000}, {9, 100000}, {25, 100000}, {18, 100000}, {31, 100000}, {17, 100000}, {12, 100000}, {36, 100000}, {30, 100000}, {13, 100000}, {27, 100000}, {10, 100000}, {21, 100000}, {38, 100000}, {11, 100000}, {46, 100000}, {2, 100000}, {33, 100000}, {20, 100000}, {8, 100000}, {35, 100000}, {43, 100000}, {23, 100000}, {22, 100000}, {15, 100000}, {44, 100000}, {45, 100000}, {7, 100000}, {24, 100000}, {42, 100000}, {6, 100000}, {1, 100000}, {26, 100000}, {39, 100000}, {32, 100000}, {40, 100000}, {41, 100000}, {4, 100000}},
		23))
}
