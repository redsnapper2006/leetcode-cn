package main

import (
	"fmt"
	"math"
)

func kthPalindrome(queries []int, intLength int) []int64 {
	times := (intLength + 1) / 2

	ret := []int64{}
	for _, q := range queries {
		t := q - 1

		bb := []int{}
		for i := 0; i < times; i++ {
			m := t % 10
			bb = append(bb, m)
			t /= 10
		}

		if t > 0 || bb[len(bb)-1] == 9 {
			ret = append(ret, -1)
			continue
		}

		bb[len(bb)-1] += 1

		var aggr int64 = 0
		for i := len(bb) - 1; i >= 0; i-- {
			aggr = aggr*10 + int64(bb[i])
		}
		idx := 0
		if intLength%2 == 1 {
			idx = 1
		}
		for i := idx; i < len(bb); i++ {
			aggr = aggr*10 + int64(bb[i])
		}
		ret = append(ret, aggr)
	}
	return ret
}

func kthPalindrome2(queries []int, intLength int) []int64 {
	ans := make([]int64, len(queries))
	base := int(math.Pow10((intLength - 1) / 2))
	for i, q := range queries {
		if q > 9*base {
			ans[i] = -1
			continue
		}
		v := base + q - 1 // 回文数左半部分
		x := v
		if intLength%2 == 1 {
			x /= 10
		} // 去掉最低位
		for ; x > 0; x /= 10 {
			v = v*10 + x%10 // 翻转 x 到 v 后
		}
		ans[i] = int64(v)
	}
	return ans
}

func main() {
	fmt.Println(kthPalindrome([]int{875, 90, 209, 416, 62, 647, 398, 909, 669, 186, 492, 748, 662, 80, 414, 550, 866, 358, 744, 478, 19, 637, 501, 129, 635, 358, 867, 723, 874, 454, 882, 406, 360, 516, 632, 883, 771, 21, 358, 147, 109, 472, 447, 493903904, 808, 94, 645, 707, 98043237, 573, 508, 142, 142, 855, 498, 56, 993, 355, 572, 788, 977, 646, 279, 821, 530, 726, 631, 61, 362, 136, 814, 357, 105, 829909848, 645, 855, 862, 635, 451, 888, 609788691, 961592349, 386, 884, 536, 334, 585, 71, 612}, 15))
}
