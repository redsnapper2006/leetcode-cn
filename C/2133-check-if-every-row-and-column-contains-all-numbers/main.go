package main

func checkValid(matrix [][]int) bool {
	n := len(matrix)
	for i := 0; i < len(matrix); i++ {
		m := map[int]int{}
		for j := 0; j < len(matrix[i]); j++ {
			if matrix[i][j] > n || matrix[i][j] < 1 {
				return false
			}
			m[matrix[i][j]]++
		}
		if len(m) != n {
			return false
		}
	}
	for i := 0; i < len(matrix[0]); i++ {
		m := map[int]int{}
		for j := 0; j < len(matrix); j++ {
			if matrix[j][i] > n || matrix[j][i] < 1 {
				return false
			}
			m[matrix[j][i]]++
		}
		if len(m) != n {
			return false
		}
	}
	return true
}

func main() {

}
