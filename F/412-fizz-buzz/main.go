package main

import (
	"fmt"
	"strconv"
)

func fizzBuzz(n int) []string {
	var b []string
	for i := 1; i <= n; i++ {
		t := ""
		if i%15 == 0 {
			t = "FizzBuzz"
		} else if i%3 == 0 {
			t = "Fizz"
		} else if i%5 == 0 {
			t = "Buzz"
		} else {
			t = strconv.Itoa(i)
		}
		b = append(b, t)
	}
	return b
}

func main() {
	fmt.Println("a")
}
