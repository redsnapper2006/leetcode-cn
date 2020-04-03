package main

func search(nums []int, target int) int {
	s, e := 0, len(nums)-1
	for s < e {
		m := s + (e-s)/2
		if nums[m] == target {
			return m
		} else if nums[m] < target {
			s = m + 1
		} else {
			e = m - 1
		}
	}
	if nums[s] == target {
		return s
	}
	return -1
}

func main() {
	fmt.Println("a")
}
