package main

func quickSelect(nums []int, k, start, end int) int {

	pivot := nums[start]

	left, right := start+1, end
	for left <= right {
		for left <= right && nums[left] < pivot {
			left++
		}
		for left <= right && nums[right] >= pivot {
			right--
		}
		if left <= right {
			nums[left], nums[right] = nums[right], nums[left]
		}
	}
	nums[right], nums[start] = nums[start], nums[right]

	m := right - start + 1

	if m == k {
		return pivot
	} else if k < m {
		return quickSelect(nums, k, start, left-1)
	} else {
		return quickSelect(nums, k-m, left, end)
	}
}

func wiggleSort(nums []int) {
	if len(nums) <= 1 {
		return
	}

	median := quickSelect(nums, (len(nums)+1)/2, 0, len(nums)-1)

	idx := func(i, N int) int {
		return (1 + 2*(i)) % N
	}
	N := len(nums)/2*2 + 1

	left, i, right := 0, 0, len(nums)-1
	for i <= right {
		if nums[idx(i, N)] > median {
			nums[idx(left, N)], nums[idx(i, N)] = nums[idx(i, N)], nums[idx(left, N)]
			left += 1
			i += 1
		} else if nums[idx(i, N)] < median {
			nums[idx(i, N)], nums[idx(right, N)] = nums[idx(right, N)], nums[idx(i, N)]
			right -= 1
		} else {
			i++
		}
	}
}

func main() {
	wiggleSort([]int{5, 3, 1, 2, 6, 7, 8, 5, 5})
	wiggleSort([]int{1, 3, 2, 2, 3, 1})
}
