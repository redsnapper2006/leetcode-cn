package main

import "fmt"

func knightProbability(N int, K int, r int, c int) float64 {
	buf := make([][][]float64, N)
	for i := 0; i < N; i++ {
		buf[i] = make([][]float64, N)
		for j := 0; j < N; j++ {
			buf[i][j] = make([]float64, K+1)
			buf[i][j][0] = 1
		}
	}

	cords := [][]int{{-1, -2}, {-2, -1}, {-2, 1}, {-1, 2}, {1, 2}, {2, 1}, {2, -1}, {1, -2}}

	for i := 1; i <= K; i++ {
		for m := 0; m < N; m++ {
			for n := 0; n < N; n++ {
				cnt := float64(0)
				for _, c := range cords {
					nm, nn := m+c[0], n+c[1]
					if nm >= 0 && nm < N && nn >= 0 && nn < N {
						cnt += buf[nm][nn][i-1] / 8
					}
				}
				buf[m][n][i] = cnt
			}
		}
	}

	return buf[r][c][K]
}

func main() {
	fmt.Println(knightProbability(8, 30, 6, 4))
}
