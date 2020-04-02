package main

import "fmt"

func floodFill(image [][]int, sr int, sc int, newColor int) [][]int {
	rows := len(image)
	cols := len(image[0])

	oldColor := image[sr][sc]

	M := make(map[int]bool)
	M[sr*cols+sc] = true
	image[sr][sc] = newColor

	buf := [][]int{[]int{sr, sc}}
	for len(buf) > 0 {
		var t [][]int
		for i := 0; i < len(buf); i++ {
			p := buf[i]
			r, c := p[0], p[1]
			if r > 0 && image[r-1][c] == oldColor {
				if _, ok := M[(r-1)*cols+c]; !ok {
					M[(r-1)*cols+c] = true
					image[r-1][c] = newColor
					t = append(t, []int{r - 1, c})
				}
			}
			if c > 0 && image[r][c-1] == oldColor {
				if _, ok := M[r*cols+c-1]; !ok {
					M[r*cols+c-1] = true
					image[r][c-1] = newColor
					t = append(t, []int{r, c - 1})
				}
			}
			if r < rows-1 && image[r+1][c] == oldColor {
				if _, ok := M[(r+1)*cols+c]; !ok {
					M[(r+1)*cols+c] = true
					image[r+1][c] = newColor
					t = append(t, []int{r + 1, c})
				}
			}
			if c < cols-1 && image[r][c+1] == oldColor {
				if _, ok := M[r*cols+c+1]; !ok {
					M[r*cols+c+1] = true
					image[r][c+1] = newColor
					t = append(t, []int{r, c + 1})
				}
			}
		}
		buf = t
	}

	return image
}

func main() {
	a := [][]int{[]int{0, 0, 0}, []int{0, 0, 0}}
	fmt.Println(floodFill(a, 1, 0, 2))
}
