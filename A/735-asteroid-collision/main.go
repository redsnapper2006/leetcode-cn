package main

func asteroidCollision(asteroids []int) []int {
	buf := make([]int, len(asteroids))
	copy(buf, asteroids)

	for {
		isFound := false

		t := []int{}
		i := 0
		for i < len(buf) {
			if i < len(buf)-1 && buf[i] > 0 && buf[i+1] < 0 {
				if buf[i] > -buf[i+1] {
					t = append(t, buf[i])
				} else if buf[i] < -buf[i+1] {
					t = append(t, buf[i+1])
				}
				isFound = true
				i++
			} else {
				t = append(t, buf[i])
			}
			i++
		}

		if !isFound {
			break
		}
		buf = t
	}

	return buf
}
