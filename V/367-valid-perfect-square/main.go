package main

import "fmt"

func isPerfectSquare(num int) bool {
	s, e := 1, num/2+1

	for s <= e {
		m := s + (e-s)/2
		if m*m == num {
			return true
		} else if m*m > num {
			e = m - 1
		} else {
			s = m + 1
		}
	}

	return false
}

func main() {
	fmt.Println(isPerfectSquare(17))
	fmt.Println(isPerfectSquare(1))
}
