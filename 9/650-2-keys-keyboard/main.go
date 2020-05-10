package main

func minSteps(n int) int {
	if n == 1 {
		return 0
	}
	if n == 2 {
		return n
	}
	min := 1<<31 - 1
	for i := 2; i <= n/2+1; i++ {
		if n%i == 0 {
			factor := minSteps(i)
			if min > factor+(n/i) {
				min = factor + (n / i)
			}
		}
	}
	if min == 1<<31-1 {
		return n
	}
	return min
}

func main() {
	fmt.Println("a")
}
