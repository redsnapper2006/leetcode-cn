package main

import "fmt"

func profitableSchemes(n int, minProfit int, group []int, profit []int) int {
	buf := make([][][]int, len(group)+1)
	for i := 0; i < len(group)+1; i++ {
		buf[i] = make([][]int, n+1)
		for j := 0; j < len(buf[i]); j++ {
			buf[i][j] = make([]int, minProfit+1)
		}
	}
	buf[0][0][0] = 1

	for i := 0; i < len(group); i++ {
		for j := 0; j < n+1; j++ {
			for m := 0; m < minProfit+1; m++ {
				if j < group[i] {
					buf[i+1][j][m] = buf[i][j][m]
				} else {
					idx := m - profit[i]
					if idx < 0 {
						idx = 0
					}
					buf[i+1][j][m] = (buf[i][j][m] + buf[i][j-group[i]][idx]) % 1000000007
				}
			}
		}
	}
	// fmt.Println(buf)

	ret := 0
	for i := 0; i < len(buf[len(group)]); i++ {
		ret += buf[len(group)][i][minProfit]
		ret %= 1000000007
	}

	return ret
}

func profitableSchemesV2(n int, minProfit int, group []int, profit []int) int {
	buf := make([]map[int]int, n+1)
	for i := 0; i < n+1; i++ {
		buf[i] = map[int]int{}
	}
	buf[0] = map[int]int{0: 1}

	for i := 0; i < len(group); i++ {
		t := [][]int{}
		for j := 0; j < len(buf); j++ {
			if j+group[i] > n {
				break
			}
			for k, v := range buf[j] {
				t = append(t, []int{j + group[i], k + profit[i], v})
			}
		}

		for j := 0; j < len(t); j++ {
			buf[t[j][0]][t[j][1]] += t[j][2]
			buf[t[j][0]][t[j][1]] %= 1000000007
		}
	}
	// fmt.Println(buf)

	ret := 0
	for k, v := range buf {
		if k > n {
			continue
		}
		for k2, v2 := range v {
			if k2 < minProfit {
				continue
			}
			ret += v2
			ret %= 1000000007
		}
	}

	return ret
}

func profitableSchemesV3(n int, minProfit int, group []int, profit []int) int {
	ret := 0

	var recur func(n, minProfit, start, currentN, currentProfit int, group []int, profit []int, ret *int)
	recur = func(n, minProfit, start, currentN, currentProfit int, group []int, profit []int, ret *int) {
		// fmt.Println(start, currentN, currentProfit, *ret)
		// fmt.Println("min", minProfit, currentProfit)
		if currentN > n || start > len(group) {
			return
		}
		if currentProfit >= minProfit {
			// fmt.Println("hit", start, currentN, currentProfit, *ret)
			*ret++
		}
		for i := start; i < len(group); i++ {
			recur(n, minProfit, i+1, currentN+group[i], currentProfit+profit[i], group, profit, ret)
		}
	}

	recur(n, minProfit, 0, 0, 0, group, profit, &ret)
	return ret
}

func main() {
	fmt.Println(profitableSchemes(5, 3, []int{2, 2}, []int{2, 3}))
	fmt.Println(profitableSchemes(95, 53, []int{82, 7, 18, 34, 1, 3, 83, 56, 50, 34, 39, 38, 76, 92, 71, 2, 6, 74, 1, 82, 22, 73, 88, 98, 6, 71, 6, 26, 100, 75, 57, 88, 43, 16, 22, 89, 7, 9, 78, 97, 22, 87, 34, 81, 74, 56, 49, 94, 87, 71, 59, 6, 20, 66, 64, 37, 2, 42, 30, 87, 73, 16, 39, 87, 28, 9, 95, 78, 43, 59, 87, 78, 2, 93, 7, 22, 21, 59, 68, 67, 65, 63, 78, 20, 82, 35, 86}, []int{45, 57, 38, 64, 52, 92, 31, 57, 31, 52, 3, 12, 93, 8, 11, 60, 55, 92, 42, 27, 40, 10, 77, 53, 8, 34, 87, 39, 8, 35, 28, 70, 32, 97, 88, 54, 82, 54, 54, 10, 78, 23, 82, 52, 10, 49, 8, 36, 9, 52, 81, 26, 5, 2, 30, 39, 89, 62, 39, 100, 67, 33, 86, 22, 49, 15, 94, 59, 47, 41, 45, 17, 99, 87, 77, 48, 22, 77, 82, 85, 97, 66, 3, 38, 49, 60, 66}))

	fmt.Println(profitableSchemes(1, 1, []int{2, 2, 2, 2, 2}, []int{1, 2, 1, 1, 0}))

	fmt.Println(profitableSchemes(100, 100, []int{6, 37, 1, 24, 35, 31, 1, 2, 15, 4, 7, 3, 1, 1, 1, 2, 7, 13, 6, 2, 21, 31, 4, 7, 7, 8, 7, 4, 15, 2, 1, 24, 27, 8, 30, 10, 8, 6, 4, 9, 2, 6, 12, 3, 4, 11, 8, 29, 3, 1, 8, 4, 1, 6, 1, 9, 3, 11, 1, 5, 1, 1, 3, 8, 7, 4, 15, 2, 3, 15, 9, 2, 12, 12, 11, 3, 10, 5, 12, 13, 7, 33, 10, 42, 11, 2, 9, 32, 1, 1, 3, 44, 7, 15, 4, 1, 9, 19, 6, 15}, []int{2, 7, 1, 29, 1, 5, 0, 0, 1, 0, 14, 4, 7, 0, 22, 2, 0, 13, 4, 6, 13, 9, 3, 0, 9, 7, 21, 21, 23, 1, 13, 10, 5, 13, 2, 15, 3, 5, 6, 20, 11, 1, 14, 36, 0, 20, 12, 14, 0, 5, 8, 4, 24, 18, 12, 23, 7, 6, 7, 4, 12, 24, 5, 6, 25, 9, 11, 21, 13, 10, 49, 2, 0, 1, 6, 1, 18, 35, 1, 19, 0, 17, 2, 39, 21, 3, 34, 8, 13, 13, 0, 0, 4, 20, 35, 2, 31, 8, 3, 70}))

}
