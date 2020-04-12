package main

import (
	"fmt"
	"sort"
)

func intersection(start1 []int, end1 []int, start2 []int, end2 []int) []float64 {

	x11, x12 := start1[0], end1[0]
	x21, x22 := start2[0], end2[0]
	y11, y12 := start1[1], end1[1]
	y21, y22 := start2[1], end2[1]
	if x11 == x12 {
		if x21 == x22 {
			if x11 == x21 {
				if (y11 < y21 && y12 < y21 && y11 < y22 && y12 < y22) || (y21 < y11 && y22 < y11 && y21 < y12 && y22 < y12) {
					return []float64{}
				} else {
					t := []int{y11, y12, y21, y22}
					sort.Ints(t)
					return []float64{float64(x11), float64(t[1])}
				}
			} else {
				return []float64{}
			}
		} else {
			if (x11 <= x22 && x11 >= x21) || (x11 <= x21 && x11 >= x22) {
				a := float64(y21-y22) / float64(x21-x22)
				b := float64(y21) - a*float64(x21)
				y := a*float64(x11) + b
				if (y <= float64(y11) && y >= float64(y12)) || (y <= float64(y12) && y >= float64(y11)) {
					return []float64{float64(x11), y}
				} else {
					return []float64{}
				}
			} else {
				return []float64{}
			}
		}
	} else {
		if x21 == x22 {
			if (x21 <= x12 && x21 >= x11) || (x21 <= x11 && x21 >= x12) {
				a := float64(y11-y12) / float64(x11-x12)
				b := float64(y11) - a*float64(x11)
				y := a*float64(x21) + b
				if (y <= float64(y21) && y >= float64(y22)) || (y <= float64(y22) && y >= float64(y21)) {
					return []float64{float64(x21), y}
				} else {
					return []float64{}
				}
			} else {
				return []float64{}
			}
		} else {

			a1 := float64(y11-y12) / float64(x11-x12)
			b1 := float64(y11) - a1*float64(x11)
			a2 := float64(y21-y22) / float64(x21-x22)
			b2 := float64(y21) - a2*float64(x21)
			if a1 == a2 {
				if b1 != b2 {
					return []float64{}
				} else {
					if (y11 < y21 && y12 < y21 && y11 < y22 && y12 < y22) || (y21 < y11 && y22 < y11 && y21 < y12 && y22 < y12) {
						return []float64{}
					} else {
						t := []int{x11, x12, x21, x22}
						sort.Ints(t)
						return []float64{float64(t[1]), a1*float64(t[1]) + b1}
					}
				}
			} else {
				// a1*x + b1 = a2*x + b2
				x := (b2 - b1) / (a1 - a2)
				y := a1*x + b1

				if ((x <= float64(x11) && x >= float64(x12)) || (x <= float64(x12) && x >= float64(x11))) && ((x <= float64(x21) && x >= float64(x22)) || (x <= float64(x22) && x >= float64(x21))) &&
					((y <= float64(y11) && y >= float64(y12)) || (y <= float64(y12) && y >= float64(y11))) && ((y <= float64(y21) && y >= float64(y22)) || (y <= float64(y22) && y >= float64(y21))) {
					return []float64{x, y}
				} else {
					return []float64{}
				}
			}
		}
	}
	// return []float64{}
}

func main() {
	fmt.Println(intersection([]int{0, 0}, []int{1, -1},
		[]int{0, 0}, []int{-1, 1}))
}
