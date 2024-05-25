func canMakeSquare(grid [][]byte) bool {
	for r := 1; r < 3; r++ {
		for c := 1; c < 3; c++ {
			b := 0
			w := 0
			for _, cord := range [][]int{{-1, -1}, {-1, 0}, {0, -1}, {0, 0}} {
				if grid[r+cord[0]][c+cord[1]] == 'B' {
					b++
				} else {
					w++
				}
			}
			if b <= 1 || w <= 1 {
				return true
			}
		}
	}
	return false
}
