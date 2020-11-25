package main

import "fmt"

func readBinaryWatch(num int) []string {
	var recur func(from int, remain int) [][]int
	recur = func(from int, remain int) [][]int {
		if from+1 < remain {
			return nil
		}
		if remain == 0 {
			var t []int
			for i := from; i >= 0; i-- {
				t = append(t, 0)
			}
			return [][]int{t}
		}
		var ret [][]int
		for i := from; i >= 0; i-- {
			r := recur(i-1, remain-1)
			if r == nil {
				break
			}

			for _, v := range r {
				var t []int
				for j := from; j > i; j-- {
					t = append(t, 0)
				}
				t = append(t, 1)
				for j := 0; j < len(v); j++ {
					t = append(t, v[j])
				}
				ret = append(ret, t)
			}
		}
		return ret
	}
	var ret []string
	for i := 0; i <= num; i++ {
		hoursCount := i
		minutesCount := num - i
		hoursArr := recur(3, hoursCount)
		minutesArr := recur(5, minutesCount)
		var hours []int
		for _, v := range hoursArr {
			sum := 0
			for _, vv := range v {
				sum = sum*2 + vv
			}
			if sum > 11 {
				continue
			}
			hours = append(hours, sum)
		}
		var minutes []int
		for _, v := range minutesArr {
			sum := 0
			for _, vv := range v {
				sum = sum*2 + vv
			}
			if sum > 59 {
				continue
			}
			minutes = append(minutes, sum)
		}
		for _, h := range hours {
			for _, m := range minutes {
				ret = append(ret, fmt.Sprintf("%d:%02d", h, m))
			}
		}
	}

	return ret
}

func main() {
	fmt.Println(readBinaryWatch(3))
}
