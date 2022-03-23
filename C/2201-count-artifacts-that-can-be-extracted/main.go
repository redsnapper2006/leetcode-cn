package main

func digArtifacts(n int, artifacts [][]int, dig [][]int) int {
	buf := make([][]int, n)
	for i := 0; i < n; i++ {
		buf[i] = make([]int, n)
	}
	for _, d := range dig {
		r, c := d[0], d[1]
		buf[r][c] = 1
	}

	cnt := 0
	for _, artifact := range artifacts {
		r1, c1, r2, c2 := artifact[0], artifact[1], artifact[2], artifact[3]
		isVisible := true
		for i := r1; i <= r2; i++ {
			for j := c1; j <= c2; j++ {
				if buf[i][j] == 0 {
					isVisible = false
				}
			}
		}
		if isVisible {
			cnt++
		}
	}
	return cnt

}
