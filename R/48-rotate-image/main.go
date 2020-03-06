package main

import "fmt"

func rotate(matrix [][]int) {
	for i := 0; i < len(matrix); i++ {
		s, e := 0, len(matrix[0])-1
		for s < e {
			matrix[i][s], matrix[i][e] = matrix[i][e], matrix[i][s]
			s++
			e--
		}
	}
	for i := 0; i < len(matrix); i++ {
		for j := 0; j < len(matrix[0]); j++ {
			if i+j < len(matrix)-1 {
				matrix[i][j], matrix[len(matrix)-1-j][len(matrix)-1-i] = matrix[len(matrix)-1-j][len(matrix)-1-i], matrix[i][j]
			}
		}
	}
}

func main() {
	fmt.Println("a")
}
