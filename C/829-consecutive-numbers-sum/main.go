package main

func consecutiveNumbersSum(n int) int {
	ret := 0
	for k := 1; k*(k+1) <= n*2; k++ {
		valid := false

		if k%2 == 1 {
			valid = n%k == 0
		} else {
			valid = n%k != 0 && 2*n%k == 0
		}
		if valid {
			ret++
		}
	}
	return ret
}
