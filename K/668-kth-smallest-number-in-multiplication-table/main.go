package main

func findKthNumber(m int, n int, k int) int {
	l, r := 1, m*n
	for l < r {
		mid := l + (r-l)/2

		sum := 0
		row, col := m, 1
		for row >= 1 && col <= n {
			if mid >= row*col {
				sum += row
				col++
			} else {
				row--
			}
		}

		if sum < k {
			l = mid + 1
		} else {
			r = mid
		}
	}
	return l
}

func findKthNumber2(m int, n int, k int) int {
	buf := make([]int, m)
	for i := 0; i < m; i++ {
		buf[i] = 1
	}

	steps := 0
	max := m*n + 1
	for steps < k {
		max = m*n + 1
		for i := 0; i < m; i++ {
			if buf[i] <= n && max > (i+1)*buf[i] {
				max = (i + 1) * buf[i]
			}
		}
		for i := 0; i < m; i++ {
			if buf[i] <= n && max == (i+1)*buf[i] {
				steps++
				buf[i]++
			}
		}
		// fmt.Println(max, buf)
	}
	return max
}
