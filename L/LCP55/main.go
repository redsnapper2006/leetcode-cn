package main

func getMinimumTime(time []int, fruits [][]int, limit int) int {
	sum := 0

	for i := 0; i < len(fruits); i++ {
		div := fruits[i][1] / limit
		if fruits[i][1]%limit > 0 {
			div++
		}
		sum += div * time[fruits[i][0]]
	}
	return sum
}
