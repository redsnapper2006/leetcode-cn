package main

import (
	"fmt"
	"strconv"
	"strings"
)

func compareVersion(version1 string, version2 string) int {
	a1 := strings.Split(version1, ".")
	a2 := strings.Split(version2, ".")

	for i := 0; i < len(a1) && i < len(a2); i++ {
		n1, _ := strconv.ParseInt(a1[i], 10, 64)
		n2, _ := strconv.ParseInt(a2[i], 10, 64)
		if n1 < n2 {
			return -1
		} else if n1 > n2 {
			return 1
		}
	}
	if len(a1) < len(a2) {
		for i := len(a1); i < len(a2); i++ {
			n1, _ := strconv.ParseInt(a2[i], 10, 64)
			if n1 > 0 {
				return -1
			}
		}
		return 0
	} else if len(a1) > len(a2) {
		for i := len(a2); i < len(a1); i++ {
			n1, _ := strconv.ParseInt(a1[i], 10, 64)
			if n1 > 0 {
				return 1
			}
		}
		return 0
	}
	return 0
}

func main() {
	fmt.Println("a")
}
