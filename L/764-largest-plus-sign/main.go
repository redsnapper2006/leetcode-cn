package main

import (
	"fmt"
	"sort"
)

func orderOfLargestPlusSign(n int, mines [][]int) int {
	rows, cols := map[int][]int{}, map[int][]int{}
	minesMap := map[int]int{}
	for _, mine := range mines {
		minesMap[mine[0]*n+mine[1]] = 1
		_, ok := rows[mine[0]]
		if !ok {
			rows[mine[0]] = []int{}
		}
		rows[mine[0]] = append(rows[mine[0]], mine[1])
		_, ok2 := cols[mine[1]]
		if !ok2 {
			cols[mine[1]] = []int{}
		}
		cols[mine[1]] = append(cols[mine[1]], mine[0])
	}
	for _, v := range rows {
		sort.Ints(v)
	}
	for _, v := range cols {
		sort.Ints(v)
	}
	// fmt.Println("rows", rows, "cols", cols)

	ret := 0
	for i := 0; i < n; i++ {
		for j := 0; j < n; j++ {
			_, ok := minesMap[i*n+j]
			if ok {
				continue
			}
			// fmt.Println(" cord", i, j)

			left, right, top, bottom := 0, 0, 0, 0

			row, ok := rows[i]
			if !ok {
				left = j
				right = n - 1 - j
			} else {
				offset := 0
				for m := len(row) - 1; m >= 0; m-- {
					if row[m] < j {
						offset = row[m] + 1
						break
					}
				}
				left = j - offset
				offset = n - 1
				for m := 0; m < len(row); m++ {
					if row[m] > j {
						offset = row[m] - 1
						break
					}
				}
				right = offset - j
			}

			col, ok := cols[j]
			if !ok {
				top = i
				bottom = n - 1 - i
			} else {
				offset := 0
				for m := len(col) - 1; m >= 0; m-- {
					if col[m] < i {
						offset = col[m] + 1
						break
					}
				}
				top = i - offset
				offset = n - 1
				for m := 0; m < len(col); m++ {
					if col[m] > i {
						offset = col[m] - 1
						break
					}
				}
				bottom = offset - i
			}
			// fmt.Println(left, right, top, bottom)
			val := left
			if val > right {
				val = right
			}
			if val > top {
				val = top
			}
			if val > bottom {
				val = bottom
			}
			if val+1 > ret {
				ret = val + 1
			}
		}
	}

	return ret
}

func main() {
	fmt.Println(orderOfLargestPlusSign(5, [][]int{{0, 2}, {0, 4}, {1, 2}, {2, 0}, {2, 3}, {2, 4}, {3, 4}, {4, 2}, {4, 4}}))
}
