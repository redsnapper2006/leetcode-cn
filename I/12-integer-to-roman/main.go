package main

import "fmt"

func intToRoman(num int) string {

	AN := []int{1, 4, 5, 9, 10, 40, 50, 90, 100, 400, 500, 900, 1000}
	AS := []string{"I", "IV", "V", "IX", "X", "XL", "L", "XC", "C", "CD", "D", "CM", "M"}

	N := num
	s := ""
	for N > 0 {
		idx := -1
		for i := len(AN) - 1; i >= 0; i-- {
			if N >= AN[i] {
				idx = i
				break
			}
		}
		s += AS[idx]
		N -= AN[idx]
	}

	return s
}

func main() {
	fmt.Println(intToRoman(1994))
}
