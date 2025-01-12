package main

import "fmt"

func minFlipsMonoIncr(s string) int {

	barr := []byte(s)
	sidx, eidx := 0, len(barr)-1
	for sidx < len(barr) && barr[sidx] == '0' {
		sidx++
	}
	if sidx == len(barr) {
		return 0
	}
	for eidx >= 0 && barr[eidx] == '1' {
		eidx--
	}
	if eidx < sidx {
		return 0
	}

	buf := []int{}
	cnt := 0
	target := byte('1')
	for i := sidx; i <= eidx; i++ {
		if barr[i] == target {
			cnt++
		} else {
			buf = append(buf, cnt)
			cnt = 1
			target = barr[i]
		}
	}
	buf = append(buf, cnt)

	odd, even := 0, 0
	oddSum, evenSum := []int{}, []int{}
	for i, v := range buf {
		if i%2 == 1 {
			odd += v
			oddSum = append(oddSum, odd)
		} else {
			even += v
			evenSum = append(evenSum, even)
		}
	}

	ret := oddSum[len(oddSum)-1]
	if ret > evenSum[len(evenSum)-1] {
		ret = evenSum[len(evenSum)-1]
	}

	for i := 0; i < len(evenSum)-1; i++ {
		if ret > evenSum[i]+oddSum[len(oddSum)-1]-oddSum[i] {
			ret = evenSum[i] + oddSum[len(oddSum)-1] - oddSum[i]
		}
	}

	return ret
}

func main() {
	fmt.Println()
}
