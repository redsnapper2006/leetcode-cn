func furthestDistanceFromOrigin(moves string) int {
	cnt := 0
	sum := 0
	for _, b := range []byte(moves) {
		if b == 'R' {
			sum++
		} else if b == 'L' {
			sum--
		} else {
			cnt++
		}
	}

	if sum < 0 {
		sum = -sum
	}
	return cnt + sum
}
