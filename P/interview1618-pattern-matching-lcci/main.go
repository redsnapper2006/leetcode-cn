package main

import "fmt"

func patternMatching(pattern string, value string) bool {
	AC, BC := 0, 0
	for i := 0; i < len(pattern); i++ {
		if pattern[i] == 'a' {
			AC++
		} else {
			BC++
		}
	}

	if AC == 0 && BC == 0 {
		return len(value) == 0
	}

	if AC == 0 || BC == 0 {
		bCnt := 0
		if AC == 0 {
			bCnt = BC
		} else {
			bCnt = AC
		}

		if len(value)%bCnt != 0 {
			return false
		}
		c := len(value) / bCnt
		b := value[0:c]
		for i := c; i < len(value); i += c {
			if value[i:i+c] != b {
				return false
			}
		}
		return true
	}

	for i := 0; i <= len(value)/AC; i++ {
		remain := len(value) - i*AC
		if remain%BC != 0 {
			continue
		}
		bLen := remain / BC
		aLen := i
		a, b := "", ""
		isA, isB := true, true
		idx := 0
		isMatch := true
		for i := 0; i < len(pattern); i++ {
			var p *bool
			var bS *string
			var len int
			if pattern[i] == 'a' {
				p = &isA
				bS = &a
				len = aLen
			} else {
				p = &isB
				bS = &b
				len = bLen
			}

			if *p {
				*p = false
				*bS = value[idx : idx+len]
			} else {
				if value[idx:idx+len] != *bS {
					isMatch = false
					break
				}
			}
			idx += len
		}
		if a == b {
			continue
		}
		if isMatch {
			return true
		}
	}
	return false
}

func main() {
	fmt.Println("a")
}
