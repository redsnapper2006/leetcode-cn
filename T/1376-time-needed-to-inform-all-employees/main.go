package main

import "fmt"

func numOfMinutes(n int, headID int, manager []int, informTime []int) int {
	RM, TM := map[int][]int{}, map[int]int{}
	for i := 0; i < len(manager); i++ {
		_, ok := RM[manager[i]]
		if !ok {
			RM[manager[i]] = []int{}
		}
		RM[manager[i]] = append(RM[manager[i]], i)
		TM[i] = informTime[i]
	}

	buf := RM[-1]
	accum := []int{0}
	max := 0
	for len(buf) > 0 {
		var t []int
		var candi []int
		for i := 0; i < len(buf); i++ {
			candi = append(candi, RM[buf[i]]...)
			for j := 0; j < len(RM[buf[i]]); j++ {
				t = append(t, accum[i]+TM[buf[i]])
			}
			if accum[i]+TM[buf[i]] > max {
				max = accum[i] + TM[buf[i]]
			}
		}
		if len(candi) == 0 {
			break
		}
		buf = candi
		accum = t
	}
	return max
}

func main() {
	fmt.Println("a")
}
