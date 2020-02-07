package main

import (
	"fmt"
)

func subtractProductAndSum(n int) int {
	add, multi := 0, 1
	for n/10 > 0 {
		m := n % 10
		add += m
		multi *= m
		n /= 10
	}
	m := n % 10
	add += m
	multi *= m
	return multi - add
}

func main() {
	fmt.Println(subtractProductAndSum(234))
	fmt.Println(subtractProductAndSum(4420))
}
