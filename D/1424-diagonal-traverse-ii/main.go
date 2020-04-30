package main

func findDiagonalOrder(nums [][]int) []int {
	if len(nums) == 0 {
		return []int{}
	}
	rowLen := len(nums)
	colLen := 0
	for i := 0; i < len(nums); i++ {
		if colLen < len(nums[i]) {
			colLen = len(nums[i])
		}
	}
	buf := make([][]int, rowLen-1+colLen-1+1)
	for i := 0; i < rowLen; i++ {
		for j := 0; j < len(nums[i]); j++ {
			buf[i+j] = append(buf[i+j], nums[i][j])
		}
	}
	var ret []int
	for i := 0; i < len(buf); i++ {
		for j := len(buf[i]) - 1; j >= 0; j-- {
			ret = append(ret, buf[i][j])
		}
	}
	return ret
}

func main() {
	fmt.Println("a")
}
