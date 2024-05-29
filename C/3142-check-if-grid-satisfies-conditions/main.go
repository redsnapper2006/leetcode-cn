func satisfiesConditions(grid [][]int) bool {
	for i := 0; i < len(grid); i++ {
		for j := 0; j < len(grid[0]); j++ {
			if i < len(grid)-1 && grid[i][j] != grid[i+1][j] || j < len(grid[0])-1 && grid[i][j] == grid[i][j+1] {
				return false
			}
		}
	}
	return true
}
