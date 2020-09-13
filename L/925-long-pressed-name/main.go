package main

import "fmt"

func isLongPressedName(name string, typed string) bool {
	nIdx, tIdx := 0, 0

	for nIdx < len(name) && tIdx < len(typed) {
		if name[nIdx] == typed[tIdx] {
			nIdx++
			tIdx++
		} else {
			if tIdx > 0 && typed[tIdx-1] == typed[tIdx] {
				tIdx++
			} else {
				return false
			}
		}
	}
	for tIdx > 0 && tIdx < len(typed) {
		if typed[tIdx-1] != typed[tIdx] {
			return false
		}
		tIdx++
	}
	return nIdx == len(name)
}

func main() {
	fmt.Println("a")
}
