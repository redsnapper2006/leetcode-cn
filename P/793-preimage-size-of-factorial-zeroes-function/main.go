package main

// from No 172
func trailingZeroes(n int) int {
	if n < 5 {
		return 0
	}

	table := []int{5, 25, 125, 625, 3125, 15625, 78125, 390625, 1953125, 9765625, 48828125, 244140625, 1220703125}
	c := 0
	for i := 0; i < len(table); i++ {
		if n >= table[i] {
			c += n / table[i]
		} else {
			break
		}
	}
	return c
}

func preimageSizeFZF(k int) int {
	s, e := 0, 5*k
	for s <= e {
		m := s + (e-s)/2
		c := trailingZeroes(m)
		if c == k {
			return 5
		}
		if c > k {
			e = m - 1
		} else {
			s = m + 1
		}
	}
	return 0
}
