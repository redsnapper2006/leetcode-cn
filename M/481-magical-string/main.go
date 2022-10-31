package main

func magicalString(n int) int {
	buf := []int{1, 2, 2}

	base, idx := 1, 2
	for n > len(buf) {
		for j := 0; j < buf[idx]; j++ {
			buf = append(buf, base)
		}
		base %= 2
		base++
		idx++
	}

	cnt := 0
	for i := 0; i < n; i++ {
		if buf[i] == 1 {
			cnt++
		}
	}
	return cnt
}
