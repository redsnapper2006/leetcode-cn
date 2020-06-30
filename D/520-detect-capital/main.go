package main

import "fmt"

func detectCapitalUse(word string) bool {
	isAllUpper := true
	for i := 0; i < len(word); i++ {
		if word[i] >= 'a' && word[i] <= 'z' {
			isAllUpper = false
			break
		}
	}

	isAllLower := true
	for i := 0; i < len(word); i++ {
		if word[i] >= 'A' && word[i] <= 'Z' {
			isAllLower = false
			break
		}
	}

	isAllInitial := true
	if word[0] >= 'A' && word[0] <= 'Z' {
		for i := 1; i < len(word); i++ {
			if word[i] >= 'A' && word[i] <= 'Z' {
				isAllInitial = false
				break
			}
		}
	} else {
		isAllInitial = false
	}

	return isAllUpper || isAllLower || isAllInitial
}

func main() {
	fmt.Println("a")
}
