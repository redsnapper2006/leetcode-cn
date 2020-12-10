package main

import "fmt"

func concatenatedBinary(n int) int {
	N := 1000000007

	sum := 0
	for i := 1; i <= n; i++ {
		b := i
		steps := 0
		for b > 0 {
			steps++
			b /= 2
		}
		sum = ((sum << steps) | i) % N
	}
	return sum
}

func concatenatedBinaryV2(n int) int {
	N := 1000000007
	sum := 0
	steps := 1
	for i := n; i > 0; i-- {
		sum += i * steps % N
		sum = sum % N
		b := i
		for b > 0 {
			steps <<= 1
			steps %= N
			b /= 2
		}
	}
	return sum
}

func main() {
	fmt.Println(concatenatedBinary(3915))
}
