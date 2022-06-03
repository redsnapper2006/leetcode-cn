package main

func hasAllCodes(s string, k int) bool {
	if len(s) <= k {
		return false
	}
	buf := make([]int, 1<<k)
	sum := 0
	for i := 0; i < k; i++ {
		sum = sum*2 + int(s[i]-'0')
	}
	buf[sum] = 1
	for i := k; i < len(s); i++ {
		sum -= int(s[i-k]-'0') * (1 << (k - 1))
		sum = sum*2 + int(s[i]-'0')
		buf[sum] = 1
	}

	for i := 0; i < (1 << k); i++ {
		if buf[i] == 0 {
			return false
		}
	}
	return true
}
