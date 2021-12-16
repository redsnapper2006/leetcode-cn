package main

import (
	"fmt"
	"math"
	"sort"
)

type Cord struct {
	A   float64
	Idx int
}

type CordArrSlice []Cord

func (p CordArrSlice) Len() int {
	return len(p)
}

func (p CordArrSlice) Swap(i, j int) {
	p[i], p[j] = p[j], p[i]
}

func (p CordArrSlice) Less(i, j int) bool {
	return p[i].A < p[j].A
}

func visiblePoints(points [][]int, angle int, location []int) int {
	angle %= 360

	dulCnt := 0
	buf := []Cord{}
	for i, p := range points {
		if p[0] == location[0] && p[1] == location[1] {
			dulCnt++
			continue
		}

		if p[0] == location[0] {
			a := 90.0
			if p[1] < location[1] {
				a = 270.0
			}
			buf = append(buf, Cord{A: a, Idx: i})
			continue
		}

		k := float64(p[1]-location[1]) / float64(p[0]-location[0])

		a := math.Atan(k) * 180 / math.Pi
		if k >= 0 {
			if p[0] < location[0] {
				a += 180.0
			}
		} else {
			a += 180.0
			if p[0] > location[0] {
				a += 180.0
			}
		}
		buf = append(buf, Cord{A: a, Idx: i})
	}

	sort.Sort(CordArrSlice(buf))

	s, e := 0, 0
	max := 0
	for s < len(buf) {
		for (buf[s].A+float64(angle) <= 360.0 && buf[s].A <= buf[e].A && buf[e].A <= buf[s].A+float64(angle)) || (buf[s].A+float64(angle) > 360.0 && (buf[s].A <= buf[e].A && buf[e].A <= 360 || buf[e].A >= 0 && buf[e].A <= buf[s].A+float64(angle)-360.0)) {
			e++
			e %= len(buf)
			if s == e {
				break
			}
		}
		diff := e - s
		if diff <= 0 {
			diff += len(buf)
		}
		if diff > max {
			max = diff
		}
		s++
	}

	return max + dulCnt
}

func main() {
	// fmt.Println(math.Atan(-1) * 180 / math.Pi)
	// fmt.Println(visiblePoints([][]int{{-1, 1}, {0, 0}, {1, -1}, {2, 0}, {2, 1}, {2, 2}, {3, 3}, {1, 4}, {0, 2}, {-1, 2}}, 90, []int{1, 1}))
	fmt.Println(visiblePoints([][]int{{2, 1}, {2, 2}, {3, 3}}, 90, []int{1, 1}))
	fmt.Println(visiblePoints([][]int{{2, 1}, {2, 2}, {3, 4}, {1, 1}}, 90, []int{1, 1}))
	fmt.Println(visiblePoints([][]int{{1, 0}, {2, 1}}, 13, []int{1, 1}))

}
