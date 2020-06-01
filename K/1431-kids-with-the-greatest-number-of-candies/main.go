package main

import "fmt"

func kidsWithCandies(candies []int, extraCandies int) []bool {
	max := -1
	for i := 0; i < len(candies); i++ {
		if candies[i] > max {
			max = candies[i]
		}
	}

	var ret []bool
	for i := 0; i < len(candies); i++ {
		r := false
		if candies[i]+extraCandies >= max {
			r = true
		}
		ret = append(ret, r)
	}
	return ret
}

func main() {
	fmt.Println("a")
}
