package main

func numTrees(n int) int {
	if n <= 1 {
		return 1
	}
	buf := make([]int, n+1)
	buf[0] = 1
	buf[1] = 1
	for i := 2; i <= n; i++ {
		c := 0
		for j := 1; j <= i; j++ {
			c += buf[j-1] * buf[i-j]
		}
		buf[i] = c
	}
	return buf[n]
}

func main() {
	fmt.Println("a")
}
