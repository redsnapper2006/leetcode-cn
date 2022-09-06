package main

func bestHand(ranks []int, suits []byte) string {
	suit := suits[0]
	isMatch := true
	for i := 1; i < len(suits); i++ {
		if suits[i] != suit {
			isMatch = false
			break
		}
	}
	if isMatch {
		return "Flush"
	}
	buf := [13]int{}
	for _, r := range ranks {
		buf[r-1]++
	}
	for i := 0; i < 13; i++ {
		if buf[i] >= 3 {
			isMatch = true
			break
		}
	}
	if isMatch {
		return "Three of a Kind"
	}
	for i := 0; i < 13; i++ {
		if buf[i] >= 2 {
			isMatch = true
			break
		}
	}
	if isMatch {
		return "Pair"
	}
	return "High Card"
}
