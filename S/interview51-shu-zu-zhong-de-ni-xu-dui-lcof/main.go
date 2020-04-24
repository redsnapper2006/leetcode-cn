package main

import "fmt"

func reversePairs(nums []int) int {
	if len(nums) <= 1 {
		return 0
	}
	ret := 0
	buf := make([]int, len(nums))
	buf[0] = nums[len(nums)-1]
	bufLen := 1
	for i := len(nums) - 2; i >= 0; i-- {
		candi := nums[i]
		s, e := 0, bufLen-1
		for s < e {
			m := s + (e-s)/2
			if buf[m] >= candi {
				e = m - 1
			} else {
				s = m + 1
			}
		}
		// fmt.Println("before", buf, s, candi)
		if buf[s] >= candi {
			ret += s
			copy(buf[s+1:], buf[s:bufLen])
			buf[s] = candi
		} else {
			ret += s + 1
			copy(buf[s+2:], buf[s+1:bufLen])
			buf[s+1] = candi
		}
		bufLen++
		// fmt.Println("after", buf, s, candi, ret)
	}
	return ret
}

func main() {
	fmt.Println(reversePairs([]int{7, 5, 6, 4}))
	fmt.Println(reversePairs([]int{1, 3, 2, 3, 1}))

}
