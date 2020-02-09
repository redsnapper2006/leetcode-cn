package main

import (
	"fmt"
)

type Item struct {
	Val   int
	Index int
	Count int
}

func count(buf []Item) []Item {
	size := len(buf)

	if size == 1 {
		buf[0].Count = 0
		return buf
	}
	if size == 2 {
		if buf[0].Val > buf[1].Val {
			buf[0].Count = 1
			buf[1].Count = 0
			t := buf[1]
			buf[1] = buf[0]
			buf[0] = t
		} else if buf[0].Val < buf[1].Val {
			buf[0].Count = 0
			buf[1].Count = 0
		}
		return buf
	}

	half := (size + 1) / 2
	first := count(buf[0:half])
	second := count(buf[half:])

	return mergeSort(first, second)
}

func mergeSort(first, second []Item) []Item {
	firstSize := len(first)
	secondSize := len(second)
	ret := make([]Item, firstSize+secondSize)

	fStart, sStart := 0, 0
	accum := 0
	for fStart < firstSize && sStart < secondSize {
		if first[fStart].Val > second[sStart].Val {
			accum++
			sStart++
		} else if first[fStart].Val <= second[sStart].Val {
			first[fStart].Count += accum
			fStart++
		}
	}

	if fStart < firstSize {
		for i := fStart; i < firstSize; i++ {
			first[i].Count += accum
		}
	}

	fStart, sStart = 0, 0
	index := 0
	for fStart < firstSize && sStart < secondSize {
		if first[fStart].Val > second[sStart].Val {
			ret[index] = second[sStart]
			index++
			sStart++
		} else if first[fStart].Val < second[sStart].Val {
			ret[index] = first[fStart]
			index++
			fStart++
		} else if first[fStart].Val == second[sStart].Val {
			ret[index] = first[fStart]
			fStart++
			index++
			ret[index] = second[sStart]
			sStart++
			index++
		}
	}
	if fStart < firstSize {
		for i := fStart; i < firstSize; i++ {
			ret[index] = first[i]
			index++
		}
	}
	if sStart < secondSize {
		for i := sStart; i < secondSize; i++ {
			ret[index] = second[i]
			index++
		}
	}
	return ret
}

func countSmaller(nums []int) []int {
	if len(nums) == 0 {
		return nil
	}
	buf := make([]Item, len(nums))
	for i := 0; i < len(nums); i++ {
		buf[i] = Item{Val: nums[i], Index: i, Count: 0}
	}

	count := count(buf)
	ret := make([]int, len(nums))
	for i := 0; i < len(count); i++ {
		ret[count[i].Index] = count[i].Count
	}
	return ret
}

func countSmallerV2(nums []int) []int {
	var ret []int
	for i := 0; i < len(nums); i++ {
		c := 0
		for j := i + 1; j < len(nums); j++ {
			if nums[i] > nums[j] {
				c++
			}
		}
		ret = append(ret, c)
	}
	return ret
}

func main() {
	// fmt.Println(countSmaller([]int{5, 2, 6, 1}))
	fmt.Println(countSmaller([]int{26, 78, 27, 100, 33, 67, 90, 23, 66, 5, 38, 7, 35, 23, 52, 22, 83, 51, 98, 69, 81, 32, 78, 28, 94, 13, 2, 97, 3, 76, 99, 51, 9, 21, 84, 66, 65, 36, 100, 41}))
}
