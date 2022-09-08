package main

func divisorSubstrings(num int, k int) int {
	buf := []int{}
	n := num
	for n > 0 {
		buf = append(buf, n%10)
		n /= 10
	}

	v := 0
	factor := 1
	for i := 0; i < k; i++ {
		v += buf[i] * factor
		factor *= 10
	}
	factor /= 10

	ret := 0
	if v > 0 && num%v == 0 {
		ret++
	}

	for i := k; i < len(buf); i++ {
		v /= 10
		v = buf[i]*factor + v

		if v > 0 && num%v == 0 {
			ret++
		}
	}
	return ret
}
