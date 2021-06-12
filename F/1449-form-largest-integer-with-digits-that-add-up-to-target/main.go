package main

import "fmt"

func largestNumber(cost []int, target int) string {
	buf := make([][]int, target+1)
	for i := 0; i <= target; i++ {
		buf[i] = []int{}
	}

	m := map[int]int{}
	for i := 0; i < len(cost); i++ {
		m[cost[i]] = i + 1
	}

	for i := 0; i <= target; i++ {
		for k, v := range m {
			if i+k > target {
				continue
			}
			if i > 0 && len(buf[i]) == 0 {
				continue
			}

			if len(buf[i+k]) > len(buf[i])+1 {
				continue
			}
			if len(buf[i+k]) > 0 && len(buf[i+k]) == len(buf[i])+1 && buf[i+k][0] > buf[i][0] && buf[i+k][0] > v {
				continue
			}

			s, e := 0, len(buf[i])-1
			for s <= e {
				m := s + (e-s)/2
				if buf[i][m] > v {
					s = m + 1
				} else {
					e = m - 1
				}
			}
			t := make([]int, len(buf[i])+1)
			copy(t, buf[i][0:s])
			t[s] = v
			copy(t[s+1:], buf[i][s:])

			if len(buf[i+k]) < len(t) {
				buf[i+k] = t
			} else if len(buf[i+k]) == len(t) {
				isReplace := true
				for j := 0; j < len(t); j++ {
					if buf[i+k][j] == t[j] {
						continue
					}
					if buf[i+k][j] > t[j] {
						isReplace = false
					}
					break
				}
				if isReplace {
					buf[i+k] = t
				}
			}
		}
	}
	ret := []byte{}
	for i := 0; i < len(buf[target]); i++ {
		ret = append(ret, byte(buf[target][i]+'0'))
	}
	if len(ret) == 0 {
		return "0"
	}
	return string(ret)
}

func main() {
	// fmt.Println(largestNumber([]int{4, 3, 2, 5, 6, 7, 2, 5, 5}, 9))
	// fmt.Println(largestNumber([]int{2, 4, 6, 2, 4, 6, 4, 4, 4}, 5))
	// fmt.Println(largestNumber([]int{37, 84, 69, 42, 86, 49, 45, 38, 81}, 627))
	fmt.Println(largestNumber([]int{37, 73, 100, 81, 51, 35, 48, 64, 97}, 738))
}
