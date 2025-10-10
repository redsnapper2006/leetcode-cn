package main

import "fmt"

func minSwaps(s string) int {
	ones, zeros := 0, 0
	for i := 0; i < len(s); i++ {
		if s[i] == byte('1') {
			ones++
		} else {
			zeros++
		}
	}
	diff := ones - zeros
	if diff < 0 {
		diff = -diff
	}
	if diff > 1 {
		return -1
	}
	if diff == 0 {
		sone, szero := 0, 0
		for i := 0; i < len(s); i++ {
			if i%2 == 0 {
				if s[i] != byte('1') {
					sone++
				}
			} else {
				if s[i] != byte('0') {
					sone++
				}
			}
		}
		for i := 0; i < len(s); i++ {
			if i%2 == 0 {
				if s[i] != byte('0') {
					szero++
				}
			} else {
				if s[i] != byte('1') {
					szero++
				}
			}
		}
		ret := sone
		if ret > szero {
			ret = szero
		}
		return ret / 2
	}

	ret := 0
	b := byte('1')
	if zeros > ones {
		b = byte('0')
	}
	for i := 0; i < len(s); i++ {
		if i%2 == 0 {
			if s[i] != b {
				ret++
			}
		} else {
			if s[i] == b {
				ret++
			}
		}
	}
	return ret / 2

}

func main() {
	// fmt.Println(minSwaps("1000101011"))
	fmt.Println(minSwaps("100"))
}
