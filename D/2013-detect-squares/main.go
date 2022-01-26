package main

import "fmt"

type DetectSquares struct {
	XM map[int]map[int]int
	YM map[int]map[int]int
}

func Constructor() DetectSquares {
	return DetectSquares{XM: map[int]map[int]int{}, YM: map[int]map[int]int{}}
}

func (this *DetectSquares) Add(point []int) {
	x, y := point[0], point[1]
	_, ok := this.XM[x]
	if !ok {
		this.XM[x] = map[int]int{}
	}
	this.XM[x][y]++
	_, ok2 := this.YM[y]
	if !ok2 {
		this.YM[y] = map[int]int{}
	}
	this.YM[y][x]++
}

func (this *DetectSquares) Count(point []int) int {
	bx, by := point[0], point[1]
	candiY := this.XM[bx]
	cnt := 0
	for y, cy := range candiY {
		if y == by {
			continue
		}
		l := y - by
		if l < 0 {
			l = -l
		}
		lt, ok := this.YM[y][bx-l]
		if ok {
			cnt += this.XM[bx-l][by] * lt * cy
		}
		rt, ok := this.YM[y][bx+l]
		if ok {
			cnt += this.XM[bx+l][by] * rt * cy
		}
	}
	return cnt
}

func main() {
	fmt.Println()
}
