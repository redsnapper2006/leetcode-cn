package main

import "fmt"

func shoppingOffers(price []int, special [][]int, needs []int) int {
	min := 1000

	validSpec := [][]int{}
	for _, spec := range special {
		sum := 0
		for i := 0; i < len(spec)-1; i++ {
			sum += price[i] * spec[i]
		}
		if sum > spec[len(spec)-1] {
			validSpec = append(validSpec, spec)
		}
	}
	init := make([]int, len(needs)+1)
	copy(init, needs)
	buf := [][]int{init}
	for len(buf) > 0 {
		t := [][]int{}
		for _, b := range buf {
			isRemain := false
			for s := 0; s < len(validSpec); s++ {
				isValid := true
				for i := 0; i < len(b)-1; i++ {
					if b[i] < validSpec[s][i] {
						isValid = false
						break
					}
				}
				if isValid {
					bb := make([]int, len(b))
					for i := 0; i < len(b)-1; i++ {
						bb[i] = b[i] - validSpec[s][i]
					}
					bb[len(bb)-1] = b[len(b)-1] + validSpec[s][len(b)-1]
					t = append(t, bb)
					isRemain = true
				}
			}
			if !isRemain {
				sum := b[len(b)-1]
				for i := 0; i < len(b)-1; i++ {
					sum += b[i] * price[i]
				}
				if sum < min {
					min = sum
				}
			}
		}
		buf = t
	}

	return min
}

func main() {
	fmt.Println(shoppingOffers([]int{4, 3, 2, 9, 8, 8},
		[][]int{{1, 5, 5, 1, 4, 0, 18}, {3, 3, 6, 6, 4, 2, 32}},
		[]int{6, 5, 5, 6, 4, 1}))
}
