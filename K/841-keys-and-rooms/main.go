package main

import (
	"fmt"
)

func canVisitAllRooms(rooms [][]int) bool {
	M := make(map[int]bool)
	M[0] = true

	var candi []int
	next := rooms[0]
	for i := 0; i < len(next); i++ {
		if _, ok := M[next[i]]; !ok {
			M[next[i]] = false
			candi = append(candi, next[i])
		}
	}

	for len(candi) > 0 {
		var b []int
		for i := 0; i < len(candi); i++ {
			v, ok := M[candi[i]]
			if !ok || v == false {
				M[candi[i]] = true
				b = append(b, rooms[candi[i]]...)
			}
		}
		candi = b
	}

	for i := 0; i < len(rooms); i++ {
		v, ok := M[i]
		if !ok || v == false {
			return false
		}
	}
	return true
}

func main() {
	fmt.Println(canVisitAllRooms([][]int{[]int{1}, []int{2}, []int{3}, []int{}}))

	fmt.Println(canVisitAllRooms([][]int{[]int{1, 3}, []int{3, 0, 1}, []int{2}, []int{0}}))

}
