package main

import "fmt"

func findContinuousSequence(target int) [][]int {
	var b [][]int

	d := 2
	for {
		base := target / d
		s, e := 0, 0
		if d%2 == 0 {
			s = (d/2 - 1) * -1
			e = d / 2
		} else {
			s = (d / 2) * -1
			e = d / 2
		}
		accum := 0
		var t []int
		for i := s; i <= e; i++ {
			accum += base + i
			t = append(t, base+i)
		}
		if accum == target {
			b = append(b, t)
		}
		fmt.Println(base, s)
		if base+s <= 1 {
			break
		}
		d++
	}
	s, e := 0, len(b)-1
	for s < e {
		b[s], b[e] = b[e], b[s]
		s++
		e--
	}
	return b
}

func main() {
	fmt.Println(findContinuousSequence(60))
}
