package main

func digitSum(s string, k int) string {
	buf := make([]int, len(s))
	for i := 0; i < len(s); i++ {
		buf[i] = int(s[i] - '0')
	}
	for len(buf) > k {
		t := []int{}
		for i := 0; i < len(buf); i = i + k {
			sum := 0
			for j := i; j < len(buf) && j < i+k; j++ {
				sum += buf[j]
			}
			if sum > 0 {
				bb := []int{}
				for sum > 0 {
					bb = append(bb, sum%10)
					sum /= 10
				}
				for i := len(bb) - 1; i >= 0; i-- {
					t = append(t, bb[i])
				}
			} else {
				t = append(t, 0)
			}
		}
		buf = t
	}
	ret := []byte{}
	for i := 0; i < len(buf); i++ {
		ret = append(ret, byte('0'+buf[i]))
	}
	return string(ret)
}
