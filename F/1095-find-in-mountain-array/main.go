package main

import (
	"fmt"
)

type MountainArray struct {
	BUF []int
}

func (this *MountainArray) get(index int) int {
	return this.BUF[index]
}
func (this *MountainArray) length() int {
	return len(this.BUF)
}

func findInMountainArray(target int, mountainArr *MountainArray) int {
	size := mountainArr.length()

	var binarySearch func(target int, mountainArr *MountainArray, start, end int, ascordesc bool) int
	binarySearch = func(target int, mountainArr *MountainArray, start, end int, ascordesc bool) int {
		if start == end {
			if mountainArr.get(start) == target {
				return start
			} else {
				return -1
			}
		}
		if start+1 == end {
			if mountainArr.get(start) == target {
				return start
			} else if mountainArr.get(start+1) == target {
				return end
			} else {
				return -1
			}
		}
		mid := (start + end) / 2
		v := mountainArr.get(mid)
		if v == target {
			return mid
		} else if v > target {
			if ascordesc {
				return binarySearch(target, mountainArr, start, mid-1, ascordesc)
			} else {
				return binarySearch(target, mountainArr, mid+1, end, ascordesc)
			}
		} else {
			if ascordesc {
				return binarySearch(target, mountainArr, mid+1, end, ascordesc)
			} else {
				return binarySearch(target, mountainArr, start, mid-1, ascordesc)
			}
		}
	}

	var searchFrom func(target int, mountainArr *MountainArray, start, end int) int
	searchFrom = func(target int, mountainArr *MountainArray, start, end int) int {
		if start == end {
			if mountainArr.get(start) == target {
				return start
			} else {
				return -1
			}
		}
		if start+1 == end {
			s1, s2 := mountainArr.get(start), mountainArr.get(end)
			if s1 == target {
				return start
			} else if s2 == target {
				return end
			} else {
				return -1
			}
		}
		if start+2 == end {
			s1, s2, s3 := mountainArr.get(start), mountainArr.get(start+1), mountainArr.get(end)
			if s1 == target {
				return start
			} else if s2 == target {
				return start + 1
			} else if s3 == target {
				return end
			} else {
				return -1
			}
		}
		mid := (start + end) / 2
		n1, n2, n3 := mountainArr.get(mid-1), mountainArr.get(mid), mountainArr.get(mid+1)

		if n1 < n2 && n2 < n3 {
			if n2 == target {
				return mid
			} else if n2 < target {
				return searchFrom(target, mountainArr, mid+1, end)
			} else {
				left := binarySearch(target, mountainArr, start, mid-1, true)
				right := searchFrom(target, mountainArr, mid+1, end)
				if left == -1 {
					return right
				} else {
					return left
				}
			}
		} else if n1 > n2 && n2 > n3 {
			if n2 == target {
				left := searchFrom(target, mountainArr, start, mid-1)
				if left == -1 {
					return mid
				} else {
					return left
				}
			} else if n2 < target {
				return searchFrom(target, mountainArr, start, mid-1)
			} else {
				left := searchFrom(target, mountainArr, start, mid-1)
				right := binarySearch(target, mountainArr, mid+1, end, false)
				if left == -1 {
					return right
				} else {
					return left
				}
			}
		} else if n1 < n2 && n2 > n3 {
			if n2 == target {
				return mid
			} else {
				left := binarySearch(target, mountainArr, start, mid-1, true)
				right := binarySearch(target, mountainArr, mid+1, end, false)
				if left == -1 {
					return right
				} else {
					return left
				}
			}
		}
		return -1
	}
	return searchFrom(target, mountainArr, 0, size-1)
}

func main() {
	ma := MountainArray{BUF: []int{1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 21, 23, 25, 27, 29, 31, 33, 35, 37, 39, 41, 43, 45, 47, 49, 51, 53, 55, 57, 59, 61, 63, 65, 67, 69, 71, 73, 75, 77, 79, 81, 83, 85, 87, 89, 91, 93, 95, 97, 99, 101, 103, 105, 107, 109, 111, 113, 115, 117, 119, 121, 123, 125, 127, 129, 131, 133, 135, 137, 139, 141, 143, 145, 147, 149, 151, 153, 155, 157, 159, 161, 163, 165, 167, 169, 171, 173, 175, 177, 179, 181, 183, 185, 187, 189, 191, 193, 195, 197, 199, 201, 199, 197, 195, 193, 191, 189, 187, 185, 183, 181, 179, 177, 175, 173, 171, 169, 167, 165, 163}}
	fmt.Println(findInMountainArray(181, &ma))
	ma = MountainArray{BUF: []int{1, 6, 11, 16, 21, 26, 31, 36, 41, 46, 51, 56, 61, 66, 71, 76, 81, 86, 91, 96, 101, 96, 91, 86, 81, 76, 71, 66, 61, 56, 51, 46, 41, 36, 31, 26, 21, 16, 11, 6}}
	fmt.Println(findInMountainArray(81, &ma))
}
