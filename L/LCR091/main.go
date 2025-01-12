package main

func minCost(costs [][]int) int {
	buf := make([][]int, len(costs))

	buf[0] = []int{costs[0][0], costs[0][1], costs[0][2]}
	for i := 1; i < len(costs); i++ {
		buf[i] = make([]int, 3)

		buf[i][0] = buf[i-1][1] + costs[i][0]
		if buf[i][0] > buf[i-1][2]+costs[i][0] {
			buf[i][0] = buf[i-1][2] + costs[i][0]
		}
		buf[i][1] = buf[i-1][0] + costs[i][1]
		if buf[i][1] > buf[i-1][2]+costs[i][1] {
			buf[i][1] = buf[i-1][2] + costs[i][1]
		}
		buf[i][2] = buf[i-1][0] + costs[i][2]
		if buf[i][2] > buf[i-1][1]+costs[i][2] {
			buf[i][2] = buf[i-1][1] + costs[i][2]
		}
	}

	ret := buf[len(buf)-1][0]
	if ret > buf[len(buf)-1][1] {
		ret = buf[len(buf)-1][1]
	}
	if ret > buf[len(buf)-1][2] {
		ret = buf[len(buf)-1][2]
	}
	return ret
}
