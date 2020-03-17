package main

func updateMatrix(matrix [][]int) [][]int {
	ret := make([][]int, len(matrix))
	for i := 0; i < len(matrix); i++ {
		ret[i] = make([]int, len(matrix[0]))
	}

	var candi [][]int
	for i := 0; i < len(matrix); i++ {
		for j := 0; j < len(matrix[0]); j++ {
			if matrix[i][j] == 0 {
				candi = append(candi, []int{i, j})
			}
		}
	}

	step := 0
	for len(candi) > 0 {
		step++
		var b [][]int
		for i := 0; i < len(candi); i++ {
			r, c := candi[i][0], candi[i][1]
			if r > 0 && matrix[r-1][c] == 1 {
				b = append(b, []int{r - 1, c})
			}
			if c > 0 && matrix[r][c-1] == 1 {
				b = append(b, []int{r, c - 1})
			}
			if r < len(matrix)-1 && matrix[r+1][c] == 1 {
				b = append(b, []int{r + 1, c})
			}
			if c < len(matrix[0])-1 && matrix[r][c+1] == 1 {
				b = append(b, []int{r, c + 1})
			}
		}

		for i := 0; i < len(b); i++ {
			r, c := b[i][0], b[i][1]
			matrix[r][c] = 0
			ret[r][c] = step
		}
		candi = b
	}

	return ret
}

func main() {
	fmt.Println("a")
}
