package main

func minBitFlips(start int, goal int) int {
	xor := start ^ goal
	count := 0
	for xor > 0 {
		if xor&1 == 1 {
			count++
		}
		xor = xor >> 1
	}
	return count
}
