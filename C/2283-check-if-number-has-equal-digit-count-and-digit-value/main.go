package main

func digitCount(num string) bool {
	m := map[int]int{}
	for _, b := range num {
		m[int(b-('0'))]++
	}

	for i := 0; i < len(num); i++ {
		if m[i] != int(num[i]-'0') {
			return false
		}
	}
	return true
}
