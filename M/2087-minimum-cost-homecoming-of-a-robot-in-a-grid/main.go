package main

func minCost(startPos []int, homePos []int, rowCosts []int, colCosts []int) int {
	sum := 0
	row := startPos[0]
	for row != homePos[0] {
		if row > homePos[0] {
			row--
		} else {
			row++
		}
		sum += rowCosts[row]
	}
	col := startPos[1]
	for col != homePos[1] {
		if col > homePos[1] {
			col--
		} else {
			col++
		}
		sum += colCosts[col]
	}
	return sum
}
