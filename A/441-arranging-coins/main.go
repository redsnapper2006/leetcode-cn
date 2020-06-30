package main

import "fmt"

func arrangeCoins(n int) int {

	i := 0
	for {
		if i*(i+1)/2 > n {
			return i - 1
		}
		i++
	}

}

func main() {
	fmt.Println("a")
}
