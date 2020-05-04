package main

func uniquePathsWithObstacles(obstacleGrid [][]int) int {
	if len(obstacleGrid) == 0 || len(obstacleGrid[0]) == 0 {
		return 0
	}
	buf := make([][]int, len(obstacleGrid))
	for i := 0; i < len(obstacleGrid); i++ {
		buf[i] = make([]int, len(obstacleGrid[0]))
	}
	for i := 0; i < len(obstacleGrid[0]); i++ {
		if obstacleGrid[0][i] == 1 {
			break
		}
		buf[0][i] = 1
	}

	for i := 1; i < len(obstacleGrid); i++ {
		for j := 0; j < len(obstacleGrid[0]); j++ {
			if obstacleGrid[i][j] == 1 {
				continue
			}
			left := 0
			if j > 0 {
				left = buf[i][j-1]
			}
			buf[i][j] = left + buf[i-1][j]
		}
	}
	return buf[len(obstacleGrid)-1][len(obstacleGrid[0])-1]
}

func main() {
	fmt.Println("a")
}
