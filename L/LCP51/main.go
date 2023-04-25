package main

func perfectMenu(materials []int, cookbooks [][]int, attribute [][]int, limit int) int {
	buf := [][]int{{0, 0, 0, 0, 0, 0, 0}}

	for i := 0; i < len(cookbooks); i++ {
		size := len(buf)
		for j := 0; j < size; j++ {
			if buf[j][0]+cookbooks[i][0] > materials[0] ||
				buf[j][1]+cookbooks[i][1] > materials[1] ||
				buf[j][2]+cookbooks[i][2] > materials[2] ||
				buf[j][3]+cookbooks[i][3] > materials[3] ||
				buf[j][4]+cookbooks[i][4] > materials[4] {
				continue
			}
			buf = append(buf, []int{buf[j][0] + cookbooks[i][0],
				buf[j][1] + cookbooks[i][1],
				buf[j][2] + cookbooks[i][2],
				buf[j][3] + cookbooks[i][3],
				buf[j][4] + cookbooks[i][4],
				buf[j][5] + attribute[i][0],
				buf[j][6] + attribute[i][1]})
		}
	}
	max := -1
	for _, b := range buf {
		if b[6] >= limit && b[5] > max {
			max = b[5]
		}
	}
	return max
}
