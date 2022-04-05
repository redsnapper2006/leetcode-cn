package main

import "sort"

func smallestNumber(num int64) int64 {
	if num == 0 {
		return num
	}
	isminus := false
	if num < 0 {
		isminus = true
	}
	buf := []int{}
	p := num
	if p < 0 {
		p = -p
	}
	for p > 0 {
		t := int(p % 10)
		buf = append(buf, t)
		p /= 10
	}
	sort.Ints(buf)

	if !isminus {
		if buf[0] == 0 {
			idx := 0
			for idx < len(buf) {
				if buf[idx] > 0 {
					break
				}
				idx++
			}
			buf[0], buf[idx] = buf[idx], buf[0]
		}
		r := int64(0)
		for _, v := range buf {
			r = r*10 + int64(v)
		}
		return r
	}
	r := int64(0)
	for i := len(buf) - 1; i >= 0; i-- {
		r = r*10 + int64(buf[i])
	}
	return -r
}
