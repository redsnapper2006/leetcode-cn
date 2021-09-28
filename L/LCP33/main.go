package main

import "fmt"

func storeWater(bucket []int, vat []int) int {
	cnt := 0
	for i := 0; i < len(bucket); i++ {
		if vat[i] == 0 {
			continue
		}
		b := bucket[i]
		if b == 0 {
			b = 1
		}
		c := vat[i] / b
		if vat[i]%b > 0 {
			c++
		}
		if c > cnt {
			cnt = c
		}
	}
	if cnt == 0 {
		return 0
	}

	buf := make([][]int, len(bucket))
	for i := 0; i < len(bucket); i++ {
		buf[i] = make([]int, cnt)
	}

	for i := 0; i < len(bucket); i++ {
		if vat[i] == 0 {
			continue
		}
		for j := 1; j <= cnt; j++ {
			t := vat[i] / j
			if vat[i]%j > 0 {
				t++
			}
			if t > bucket[i] {
				buf[i][j-1] = t - bucket[i]
			}
		}
	}
	// fmt.Println(buf)

	ret := 1000000
	for j := 0; j < cnt; j++ {
		sum := 0
		for i := 0; i < len(bucket); i++ {
			sum += buf[i][j]
		}
		if sum+j+1 < ret {
			ret = sum + j + 1
		}
	}
	return ret
}

func main() {
	fmt.Println(storeWater([]int{1, 3}, []int{6, 8}))
	fmt.Println(storeWater([]int{9, 0, 1}, []int{0, 2, 2}))
}
