package main

import "fmt"

func numberOfSteps(num int) int {
	s := 0
	for num > 0 {
		if num%2 == 0 {
			num /= 2
		} else {
			num--
		}
		s++
	}
	return s

}

func main() {
	fmt.Println("a")
}
