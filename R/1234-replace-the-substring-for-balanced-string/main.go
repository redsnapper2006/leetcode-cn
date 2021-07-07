package main

import "fmt"

func balancedString(s string) int {
	buf := make([][]int, len(s)+1)
	buf[0] = []int{0, 0, 0, 0}
	E, Q, R, W := 0, 0, 0, 0
	for i := 0; i < len(s); i++ {
		if byte(s[i]) == 'E' {
			E++
		}
		if byte(s[i]) == 'Q' {
			Q++
		}
		if byte(s[i]) == 'R' {
			R++
		}
		if byte(s[i]) == 'W' {
			W++
		}
		buf[i+1] = []int{E, Q, R, W}
	}
	// fmt.Println(buf)
	diffE, diffQ, diffR, diffW := E-len(s)/4, Q-len(s)/4, R-len(s)/4, W-len(s)/4
	diff := []int{0, 0, 0, 0}
	if diffE > 0 {
		diff[0] = diffE
	}
	if diffQ > 0 {
		diff[1] = diffQ
	}
	if diffR > 0 {
		diff[2] = diffR
	}
	if diffW > 0 {
		diff[3] = diffW
	}
	isTarget := true
	for j := 0; j < len(diff); j++ {
		if diff[j] > 0 {
			isTarget = false
			break
		}
	}
	if isTarget {
		return 0
	}

	min := len(s) + 1
	for i := 1; i < len(buf); i++ {
		isTarget := true
		for j := 0; j < len(diff); j++ {
			if buf[i][j] < diff[j] {
				isTarget = false
				break
			}
		}
		if !isTarget {
			continue
		}

		s, e := 0, i
		for s <= e {
			m := s + (e-s)/2
			isTarget := true
			for j := 0; j < len(diff); j++ {
				if diff[j] > 0 && buf[i][j]-buf[m][j] < diff[j] {
					isTarget = false
					break
				}
			}
			if isTarget {
				s = m + 1
			} else {
				e = m - 1
			}
		}
		if i-s+1 < min {
			min = i - s + 1
		}
	}

	return min
}

func main() {
	fmt.Println(balancedString("WWEQERQWQWWRWWERQWEQ"))
}
