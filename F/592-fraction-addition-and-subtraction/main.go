package main

import "fmt"




func fractionAddition(expression string) string {
	numerator := []int{}
	denominator := []int{}

	isMinus := false
	base := 0
	for _, b := range expression {
		if byte(b) == byte('-') {
			if len(numerator) > len(denominator) {
				denominator = append(denominator, base)
			}
			base = 0
			isMinus = true
			continue
		}
		if byte(b) == byte('+') {
			denominator = append(denominator, base)
			base = 0
			isMinus = false
			continue
		}
		if byte(b) == byte('/') {
			if isMinus {
				base = -base
			}
			numerator = append(numerator, base)
			isMinus = false
			base = 0
			continue
		}
		base = base*10 + int(byte(b)-byte('0'))
	}
	denominator = append(denominator, base)
	aggr := 1

	for _, deno := range denominator {
		aggr *= deno
	}

	sum := 0
	for i, nume := range numerator {
		sum += aggr / denominator[i] * nume
	}
	// fmt.Println(numerator, denominator)
	// fmt.Println(sum, aggr)

	if sum == 0 {
		return "0/1"
	}
	var GCD func(x, y int) int
	GCD = func(x, y int) int {
		p1, p2 := x, y
		if p2 > p1 {
			p1, p2 = y, x
		}
		if p1%p2 == 0 {
			return p2
		}
		return GCD(p1%p2, p2)
	}

	isMinus = false
	if sum < 0 {
		isMinus = true
		sum = -sum
	}
	g := GCD(sum, aggr)
	sum /= g
	aggr /= g
	s := fmt.Sprintf("%d/%d", sum, aggr)
	if isMinus {
		s = "-" + s
	}

	return s
}
