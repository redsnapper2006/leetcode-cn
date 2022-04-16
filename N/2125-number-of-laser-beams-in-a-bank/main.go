package main

func numberOfBeams(bank []string) int {
	prevIdx := -1
	prev := 0
	ret := 0
	for i, l := range bank {
		cnt := 0
		for _, b := range l {
			if byte(b) == byte('1') {
				cnt++
			}
		}
		if cnt > 0 {
			if i-prevIdx > 0 {
				ret += prev * cnt
			}
			prevIdx = i
			prev = cnt
		}
	}
	return ret
}
