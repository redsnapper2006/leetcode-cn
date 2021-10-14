package main

import "fmt"

func sumGame(num string) bool {
	leftSum, leftPar, rightSum, rightPar := 0, 0, 0, 0

	for i := 0; i < len(num)/2; i++ {
		if num[i] == byte('?') {
			leftPar++
		} else {
			leftSum += int(num[i] - byte('0'))
		}
	}
	for i := len(num) / 2; i < len(num); i++ {
		if num[i] == byte('?') {
			rightPar++
		} else {
			rightSum += int(num[i] - byte('0'))
		}
	}

	if (leftPar+rightPar)%2 == 1 {
		return true
	}

	diffSum := leftSum - rightSum
	diffPar := leftPar - rightPar
	return !((diffPar/2)*9 == -diffSum)
}

func main() {
	fmt.Println()
}
