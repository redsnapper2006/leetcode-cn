package main

import "fmt"

func floodFill(image [][]int, sr int, sc int, newColor int) [][]int {
	var recur func(image [][]int, sr, sc, newColor int)
	recur = func(image [][]int, sr, sc, newColor int) {
		org := image[sr][sc]
		if org == newColor {
			return
		}
		image[sr][sc] = newColor
		if sr > 0 && image[sr-1][sc] == org {
			recur(image, sr-1, sc, newColor)
		}
		if sc > 0 && image[sr][sc-1] == org {
			recur(image, sr, sc-1, newColor)
		}
		if sr < len(image)-1 && image[sr+1][sc] == org {
			recur(image, sr+1, sc, newColor)
		}
		if sc < len(image[0])-1 && image[sr][sc+1] == org {
			recur(image, sr, sc+1, newColor)
		}
	}
	recur(image, sr, sc, newColor)
	return image
}

func main() {
	fmt.Println("a")
}
