package main

import "fmt"

func getNoZeroIntegers(n int) []int {
	s := 1
	for {
		r := n - s
		t := s
		s++
		isValid := true
		for t > 0 {
			if t%10 == 0 {
				isValid = false
				break
			}
			t /= 10
		}
		if !isValid {
			continue
		}
		t = r
		isValid = true
		for t > 0 {
			if t%10 == 0 {
				isValid = false
				break
			}
			t /= 10
		}
		if !isValid {
			continue
		}
		return []int{s, r}
	}
}

func main() {
	fmt.Println("a")
}
