package main

import "fmt"

func minElements(nums []int, limit int, goal int) int {
	sum := 0
	for _, v := range nums {
		sum += v
	}

	ret := 0
	for goal != sum {
		if goal < sum {
			d := goal - sum
			c := -d
			cnt := 1
			if d < -limit {
				cnt = d / (-limit)
				c = limit
			}

			sum -= cnt * c
			ret += cnt
		} else {
			d := goal - sum
			c := d
			cnt := 1
			if d > limit {
				cnt = d / limit
				c = limit
			}
			sum += cnt * c
			ret += cnt
		}
	}
	return ret
}

func main() {
	fmt.Println("")
}
