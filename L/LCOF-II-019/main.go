package main

import "fmt"

func validPalindrome(s string) bool {
	buf := []byte(s)
	idxS, idxE := 0, len(buf)-1

	for idxS <= idxE {
		if buf[idxS] != buf[idxE] {
			break
		}
		idxS++
		idxE--
	}
	if idxS > idxE {
		return true
	}

	oneIdxS := idxS + 1
	oneIdxE := idxE
	isValid := true
	for oneIdxS <= oneIdxE {
		if buf[oneIdxS] != buf[oneIdxE] {
			isValid = false
			break
		}
		oneIdxS++
		oneIdxE--
	}
	if isValid {
		return true
	}

	twoIdxS := idxS
	twoIdxE := idxE - 1
	isValid = true
	for twoIdxS < twoIdxE {
		if buf[twoIdxS] != buf[twoIdxE] {
			isValid = false
			break
		}
		twoIdxS++
		twoIdxE--
	}
	return isValid
}

func main() {
	fmt.Println()
}
