package main

import "fmt"

func ambiguousCoordinates(s string) []string {
	trim := s[1 : len(s)-1]

	sub := func(s string) []string {
		// fmt.Println("input", s)
		if len(s) == 0 {
			return nil
		}
		if len(s) == 1 {
			return []string{s}
		}

		left, right, times := 0, 0, 1
		for i := 0; i < len(s); i++ {
			right = right*10 + int(s[i]-'0')
			times *= 10
		}
		ret := []string{}
		for i := 0; i < len(s); i++ {
			left = left*10 + int(s[i]-'0')
			right -= int(s[i]-'0') * times
			times /= 10

			if i > 0 && byte(s[0]) == '0' {
				continue
			}
			if i < len(s)-1 && byte(s[len(s)-1]) == '0' {
				continue
			}

			if left == 0 && i > 0 {
				continue
			}
			if right == 0 && i < len(s)-1 {
				continue
			}

			if i == len(s)-1 {
				ret = append(ret, s[0:i+1])
			} else {
				ret = append(ret, s[0:i+1]+"."+s[i+1:])
			}
		}
		// fmt.Println(ret)
		return ret
	}

	ret := []string{}
	for i := 0; i < len(trim); i++ {
		left, right := sub(trim[0:i]), sub(trim[i:])
		for m := 0; m < len(left); m++ {
			for n := 0; n < len(right); n++ {
				ret = append(ret, "("+left[m]+", "+right[n]+")")
			}
		}
	}
	return ret
}

func main() {
	fmt.Println(ambiguousCoordinates("(100)"))
}
