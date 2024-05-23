func distributeCandies2(n int, limit int) int {
	ans := 0

	for one := 1; one <= limit; one++ {

		for two := 1; two <= limit; two++ {
			if n-one-two <= limit {
				ans++
			}
		}
	}
	return ans
}

func cal(x int) int {
	if x < 0 {
		return 0
	}
	return x * (x - 1) / 2
}

func distributeCandies(n int, limit int) int {
	return cal(n+2) - 3*cal(n-limit+1) + 3*cal(n-(limit+1)*2+2) - cal(n-3*(limit+1)+2)
}
