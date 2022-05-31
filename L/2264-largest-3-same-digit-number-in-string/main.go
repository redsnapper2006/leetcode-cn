package main

func largestGoodInteger(num string) string {
	n := byte(' ')
	cnt := 0
	b := byte(' ')
	for i := 0; i < len(num); i++ {
		if num[i] == b {
			cnt++
			if cnt >= 3 && n < b {
				n = b
			}
		} else {
			b = num[i]
			cnt = 1
		}
	}
	if n == byte(' ') {
		return ""
	}
	return string([]byte{n, n, n})
}
