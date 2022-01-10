package main

import "fmt"

func isAdditiveNumber(num string) bool {
	var dfs func(num string, p1S, p1E, p2S, p2E int) bool
	dfs = func(num string, p1S, p1E, p2S, p2E int) bool {
		if p2E >= len(num)-1 {
			return false
		}

		if p1E > p1S && num[p1S] == byte('0') || p2E > p2S && num[p2S] == byte('0') {
			return false
		}

		l1, l2 := p1E-p1S+1, p2E-p2S+1
		tl := l1
		if tl < l2 {
			tl = l2
		}
		if len(num)-p2E-1 < tl {
			return false
		}

		buf := []int{}
		isPlus := false
		i1, i2 := p1E, p2E
		for i1 >= p1S && i2 >= p2S {
			sum := int(num[i1]-byte('0')) + int(num[i2]-byte('0'))
			if isPlus {
				sum++
			}
			isPlus = false
			if sum > 9 {
				isPlus = true
				sum -= 10
			}
			buf = append(buf, sum)
			i1--
			i2--
		}
		if i1 >= p1S {
			for i1 >= p1S {
				v := int(num[i1] - byte('0'))
				if isPlus {
					v++
				}
				isPlus = false
				if v > 9 {
					isPlus = true
					v -= 10
				}
				buf = append(buf, v)
				i1--
			}
		}
		if i2 >= p2S {
			for i2 >= p2S {
				v := int(num[i2] - byte('0'))
				if isPlus {
					v++
				}
				isPlus = false
				if v > 9 {
					isPlus = true
					v -= 10
				}
				buf = append(buf, v)
				i2--
			}
		}
		if isPlus {
			buf = append(buf, 1)
		}

		if p2E+len(buf) >= len(num) {
			return false
		}
		for i := 0; i < len(buf); i++ {
			if num[p2E+len(buf)-i] == byte('0'+buf[i]) {
				continue
			}
			return false
		}
		if p2E+len(buf) == len(num)-1 {
			return true
		}
		return dfs(num, p2S, p2E, p2E+1, p2E+len(buf))
	}

	for i := 0; i < len(num); i++ {
		for j := i + 1; j < len(num); j++ {
			r := dfs(num, 0, i, i+1, j)
			if r {
				return true
			}
		}
	}
	return false
}

func main() {
	// fmt.Println(isAdditiveNumber("112358"))
	// fmt.Println(isAdditiveNumber("199100199"))
	fmt.Println(isAdditiveNumber("8917"))
	fmt.Println(isAdditiveNumber("199001200"))
}
