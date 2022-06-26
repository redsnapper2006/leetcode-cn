package main

func minMaxGame(nums []int) int {
	cnt := len(nums)
	buf := make([]int, len(nums))
	copy(buf, nums)
	for len(buf) > 1 {
		t := make([]int, cnt/2)
		s := 0
		for i := 0; i < len(buf); i = i + 2 {
			if s%2 == 0 {
				v := buf[i]
				if v > buf[i+1] {
					v = buf[i+1]
				}
				t[i/2] = v
			} else {
				v := buf[i]
				if v < buf[i+1] {
					v = buf[i+1]
				}
				t[i/2] = v
			}
			s++
		}
		buf = t
	}
	return buf[0]
}
