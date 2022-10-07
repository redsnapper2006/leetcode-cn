package main

func minNumBooths(demand []string) int {
	ret := make([]int, 26)
	for _, d := range demand {
		buf := make([]int, 26)
		for _, b := range d {
			offset := int(byte(b) - 'a')
			buf[offset]++
		}
		for i := 0; i < 26; i++ {
			if buf[i] > ret[i] {
				ret[i] = buf[i]
			}
		}
	}
	sum := 0
	for i := 0; i < 26; i++ {
		sum += ret[i]
	}
	return sum
}
