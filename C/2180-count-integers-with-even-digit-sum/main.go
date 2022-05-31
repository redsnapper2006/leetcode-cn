package main

func countEven(num int) int {
	buf := make([]bool, num+1)
	buf[0] = true
	for i := 1; i <= num; i++ {
		buf[i] = buf[i/10] == ((i%10)%2 == 0)
	}
	cnt := 0
	for i := 1; i <= num; i++ {
		if buf[i] {
			cnt++
		}
	}
	return cnt
}
