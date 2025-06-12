package main

import "fmt"

func majorityElement(nums []int) int {
	major, cnt := 0, 0
	for _, v := range nums {
		if cnt == 0 {
			major = v
			cnt++
		} else {
			if major == v {
				cnt++
			} else {
				cnt--
			}
		}
	}
	if cnt > 0 {
		t := 0
		for _, v := range nums {
			if v == major {
				t++
			}
		}
		if t > len(nums)/2 {
			return major
		}
	}

	return -1
}

func main() {
	fmt.Println("a")
}
