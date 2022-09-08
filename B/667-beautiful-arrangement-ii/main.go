package main

func constructArray(n int, k int) []int {
	buf := make([]int, n)
	s, e := 1, 1+k
	ret := []int{}
	for s < e {
		buf[s-1] = 1
		buf[e-1] = 1
		ret = append(ret, s, e)
		s++
		e--
	}
	for i := 0; i < n; i++ {
		if buf[i] == 0 {
			ret = append(ret, i+1)
		}
	}

	return ret
}
