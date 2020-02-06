package main

import (
	"fmt"
	"math/big"
)

type UniFuncParam struct {
	A string
	B string
	X float64
}

type Point struct {
	X int
	Y int
}

func maxPoints(points [][]int) int {
	if len(points) <= 1 {
		return len(points)
	}
	store := make(map[Point]int)
	for _, v := range points {
		store[Point{X: v[0], Y: v[1]}]++
	}

	var pp []Point
	for k, _ := range store {
		pp = append(pp, k)
	}
	if len(pp) <= 1 {
		return len(points)
	}

	r := make(map[UniFuncParam][][]Point)

	for i := 0; i < len(pp); i++ {
		p1 := pp[i]
		x1, y1 := p1.X, p1.Y

		for j := i + 1; j < len(pp); j++ {
			p2 := pp[j]
			x2, y2 := p2.X, p2.Y
			a, b := big.NewRat(0, 1), big.NewRat(0, 1)
			x := 0.0

			if x1 == x2 {
				x = float64(x1)
			} else {
				a = big.NewRat(int64(y1-y2), int64(x1-x2))
				t := big.NewRat(int64(y1), int64(1))
				b = t.Sub(t, big.NewRat(1, 1).Mul(a, big.NewRat(int64(x1), int64(1))))
			}
			r[UniFuncParam{A: a.RatString(), B: b.RatString(), X: x}] = append(r[UniFuncParam{A: a.RatString(), B: b.RatString(), X: x}], []Point{p1, p2})
		}
	}

	rr := make(map[UniFuncParam]int)
	for k, v := range r {

		ppC := 0
		for i := 1; i <= len(v); i++ {
			if (i * (i + 1) / 2) == len(v) {
				ppC = i + 1
				break
			}
		}

		buf := make(map[Point]int)
		for _, ps := range v {
			p1, p2 := ps[0], ps[1]
			_, ok1 := buf[p1]
			if !ok1 {
				buf[p1]++

				ppC += store[p1] - 1

			}
			_, ok2 := buf[p2]
			if !ok2 {
				buf[p2]++

				ppC += store[p2] - 1

			}
		}
		rr[k] = ppC
	}
	max := 0
	for _, v := range rr {
		if v > max {
			max = v
		}
	}

	return max
}

func main() {
	fmt.Println(maxPoints([][]int{{1, 1}, {3, 2}, {5, 3}, {4, 1}, {2, 3}, {1, 4}}))
	// fmt.Println(maxPoints([][]int{{1, 1}, {2, 2}, {3, 3}}))
	// fmt.Println(maxPoints([][]int{{0, 0}, {1, 1}, {0, 0}}))
	fmt.Println(maxPoints([][]int{{3, 1}, {12, 3}, {3, 1}, {-6, -1}}))

}
