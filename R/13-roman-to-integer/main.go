package main

import (
	"fmt"
)

func romanToInt(s string) int {
	M := map[int]int{
		'I': 1,
		'V': 5,
		'X': 10,
		'L': 50,
		'C': 100,
		'D': 500,
		'M': 1000,
	}

	accum := 0
	for i := 0; i < len(s); i++ {
		times := 1
		if s[i] == 'I' && i < len(s)-1 && (s[i+1] == 'V' || s[i+1] == 'X') {
			times = -1
		}

		if s[i] == 'X' && i < len(s)-1 && (s[i+1] == 'L' || s[i+1] == 'C') {
			times = -1
		}
		if s[i] == 'C' && i < len(s)-1 && (s[i+1] == 'D' || s[i+1] == 'M') {
			times = -1
		}
		accum += M[int(s[i])] * times
	}
	return accum
}

func main() {
	fmt.Println(romanToInt("III"))
	fmt.Println(romanToInt("IV"))
	fmt.Println(romanToInt("IX"))
	fmt.Println(romanToInt("XXVII"))
	fmt.Println(romanToInt("LVIII"))
	fmt.Println(romanToInt("MCMXCIV"))

}
