package main

import "fmt"

func maxSumDivThree(nums []int) int {
	buf := make([]int, 3)
	for i := 0; i < len(nums); i++ {
		if nums[i]%3 == 0 {
			buf[0] += nums[i]
			if buf[1] > 0 {
				buf[1] += nums[i]
			}
			if buf[2] > 0 {
				buf[2] += nums[i]
			}

		} else if nums[i]%3 == 1 {
			a, b, c := buf[0], buf[1], buf[2]
			if c > 0 && c+nums[i] > a {
				buf[0] = c + nums[i]
			}
			if a+nums[i] > b {
				buf[1] = a + nums[i]
			}
			if b > 0 && b+nums[i] > c {
				buf[2] = b + nums[i]
			}
		} else {
			a, b, c := buf[0], buf[1], buf[2]
			if b > 0 && b+nums[i] > a {
				buf[0] = b + nums[i]
			}
			if c > 0 && c+nums[i] > b {
				buf[1] = c + nums[i]
			}
			if a+nums[i] > c {
				buf[2] = a + nums[i]
			}
		}
	}
	return buf[0]
}

func maxSumDivThreeV2(nums []int) int {
	sum := 0
	one1, one2, two1, two2 := 0, 0, 0, 0
	for i := 0; i < len(nums); i++ {
		sum += nums[i]
		if nums[i]%3 == 1 {
			if one1 == 0 {
				one1 = nums[i]
			} else {
				if one2 == 0 {
					if nums[i] < one1 {
						one2 = one1
						one1 = nums[i]
					} else {
						one2 = nums[i]
					}
				} else {
					if nums[i] < one1 {
						one2 = one1
						one1 = nums[i]
					} else if nums[i] < one2 {
						one2 = nums[i]
					}
				}
			}
		}
		if nums[i]%3 == 2 {
			if two1 == 0 {
				two1 = nums[i]
			} else {
				if two2 == 0 {
					if nums[i] < two1 {
						two2 = two1
						two1 = nums[i]
					} else {
						two2 = nums[i]
					}
				} else {
					if nums[i] < two1 {
						two2 = two1
						two1 = nums[i]
					} else if nums[i] < two2 {
						two2 = nums[i]
					}
				}
			}
		}
	}

	if sum%3 == 0 {
		return sum
	}
	if sum%3 == 1 {
		d1 := sum - one1
		if two1 > 0 && two2 > 0 && d1 < sum-two1-two2 {
			return sum - two1 - two2
		}
		return d1
	}
	d2 := sum - two1
	if one1 > 0 && one2 > 0 && d2 < sum-one1-one2 {
		return sum - one1 - one2
	}
	return d2
}

func main() {
	fmt.Println(maxSumDivThree([]int{1, 2, 3, 4, 4}))
}
