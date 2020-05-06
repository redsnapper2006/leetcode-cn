package main

func mincostTickets(days []int, costs []int) int {
	if len(days) == 0 {
		return 0
	}

	buf := make([]int, 365)
	buf[days[0]-1] = costs[0]
	base := 0
	for i := 0; i < 7 && days[0]+i-1 < 365; i++ {
		if buf[days[0]+i-1] == 0 || base+costs[1] < buf[days[0]+i-1] {
			buf[days[0]+i-1] = base + costs[1]
		}
	}
	for i := 0; i < 30 && days[0]+i-1 < 365; i++ {
		if buf[days[0]+i-1] == 0 || base+costs[2] < buf[days[0]+i-1] {
			buf[days[0]+i-1] = base + costs[2]
		}
	}

	for i := 1; i < len(days); i++ {
		base := buf[days[i-1]-1]
		if buf[days[i]-1] == 0 || base+costs[0] < buf[days[i]-1] {
			buf[days[i]-1] = base + costs[0]
		}
		for j := 0; j < 7 && days[i]+j-1 < 365; j++ {
			if buf[days[i]+j-1] == 0 || base+costs[1] < buf[days[i]+j-1] {
				buf[days[i]+j-1] = base + costs[1]
			}
		}
		for j := 0; j < 30 && days[i]+j-1 < 365; j++ {
			if buf[days[i]+j-1] == 0 || base+costs[2] < buf[days[i]+j-1] {
				buf[days[i]+j-1] = base + costs[2]
			}
		}
	}

	return buf[days[len(days)-1]-1]
}

func mincostTicketsV2(days []int, costs []int) int {
	if len(days) == 0 {
		return 0
	}

	base := days[0]
	onedayCost := costs[0] + mincostTickets(days[1:], costs)

	target := base + 7 - 1
	idx := 0
	for idx = 0; idx < len(days); idx++ {
		if days[idx] > target {
			break
		}
	}
	sevendaysCost := costs[1] + mincostTickets(days[idx:], costs)

	target = base + 30 - 1
	idx = 0
	for idx = 0; idx < len(days); idx++ {
		if days[idx] > target {
			break
		}
	}
	thirtydaysCost := costs[2] + mincostTickets(days[idx:], costs)

	ret := onedayCost
	if ret > sevendaysCost {
		ret = sevendaysCost
	}
	if ret > thirtydaysCost {
		ret = thirtydaysCost
	}
	return ret
}

func main() {
	fmt.Println("a")
}
