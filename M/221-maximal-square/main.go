package main

func maximalSquare(matrix [][]byte) int {
	if len(matrix) == 0 || len(matrix[0]) == 0 {
		return 0
	}
	createBuf := func(r, c int) [][]int {
		b := make([][]int, r)
		for i := 0; i < r; i++ {
			b[i] = make([]int, c)
		}
		return b
	}
	horizon := createBuf(len(matrix), len(matrix[0]))
	vertical := createBuf(len(matrix), len(matrix[0]))

	for i := 0; i < len(matrix); i++ {
		c := 0
		for j := len(matrix[0]) - 1; j >= 0; j-- {
			if matrix[i][j] == '1' {
				c++
				horizon[i][j] = c
			} else {
				c = 0
			}
		}
	}

	for i := 0; i < len(matrix[0]); i++ {
		c := 0
		for j := len(matrix) - 1; j >= 0; j-- {
			if matrix[j][i] == '1' {
				c++
				vertical[j][i] = c
			} else {
				c = 0
			}
		}
	}

	max := 0
	for i := 0; i < len(matrix); i++ {
		for j := 0; j < len(matrix[0]); j++ {
			h := horizon[i][j]
			v := vertical[i][j]
			s := h
			if h > v {
				s = v
			}
			if s == 0 {
				continue
			}

			if s == 1 {
				if max < 1 {
					max = 1
				}
				continue
			}
			ret := 0
			for m := 1; m < s; m++ {
				r := vertical[i][j+m]
				if horizon[i+m][j] < vertical[i][j+m] {
					r = horizon[i+m][j]
				}
				if r >= m+1 {
					ret = m + 1
				} else {
					break
				}
			}

			if max < ret*ret {
				max = ret * ret
			}
		}
	}

	return max
}

func main() {
	fmt.Println("a")
}
