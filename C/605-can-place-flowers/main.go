package main

import "fmt"

func canPlaceFlowers(flowerbed []int, n int) bool {
	candi := 0
	for i := 0; i < len(flowerbed); i++ {
		if flowerbed[i] == 1 {
			continue
		}
		previous, later := true, true
		if i > 0 && flowerbed[i-1] == 1 {
			previous = false
		}
		if i < len(flowerbed)-1 && flowerbed[i+1] == 1 {
			later = false
		}
		if previous && later {
			flowerbed[i] = 1
			candi++
		}
	}
	return candi >= n
}

func main() {
	fmt.Println("a")
}
