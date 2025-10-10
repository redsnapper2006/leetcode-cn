package main

import "fmt"

func mergeTriplets(triplets [][]int, target []int) bool {
	for i := 0; i < 3; i++ {
		isExist := false
		for j := 0; j < len(triplets); j++ {
			if triplets[j][i] == target[i] {
				isGood := true
				for m := 0; m < 3; m++ {
					if m == i {
						continue
					}
					if triplets[j][m] > target[m] {
						isGood = false
					}
				}
				if isGood {
					isExist = true
					break
				}
			}
		}
		if !isExist {
			return false
		}
	}
	return true
}

func main() {
	fmt.Println()
}
