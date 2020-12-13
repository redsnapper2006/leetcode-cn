package main

import "fmt"

func checkPalindromeFormation(a string, b string) bool {
	ba, bb := []byte(a), []byte(b)
	s, e := 0, len(a)-1
	stop := -1
	for s < e {
		if ba[s] != bb[e] {
			stop = s
			break
		}
		s++
		e--
	}
	if stop == -1 {
		return true
	}

	isMatch := true
	s, e = stop, len(a)-1-stop
	for s < e {
		if ba[s] != ba[e] {
			isMatch = false
			break
		}
		s++
		e--
	}
	if isMatch {
		return true
	}

	isMatch = true
	s, e = stop, len(a)-1-stop
	for s < e {
		if bb[s] != bb[e] {
			isMatch = false
			break
		}
		s++
		e--
	}
	if isMatch {
		return true
	}

	s, e = 0, len(a)-1
	stop = -1
	for s < e {
		if bb[s] != ba[e] {
			stop = s
			break
		}
		s++
		e--
	}
	if stop == -1 {
		return true
	}

	isMatch = true
	s, e = stop, len(a)-1-stop
	for s < e {
		if bb[s] != bb[e] {
			isMatch = false
			break
		}
		s++
		e--
	}
	if isMatch {
		return true
	}
	isMatch = true
	s, e = stop, len(a)-1-stop
	for s < e {
		if ba[s] != ba[e] {
			isMatch = false
			break
		}
		s++
		e--
	}
	if isMatch {
		return true
	}

	return false
}

func main() {
	fmt.Println(checkPalindromeFormation("abccef", "cgdhga"))
}
