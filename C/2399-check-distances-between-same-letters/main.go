package main

func checkDistances(s string, distance []int) bool {
	valid := make([]int, 26)
	pos := make([]int, 26)
	for i, b := range s {
		offset := int(byte(b) - 'a')
		valid[offset]++
		if valid[offset] == 1 {
			pos[offset] = i
		} else {
			pos[offset] = i - pos[offset] - 1
		}
	}
	// fmt.Println(valid, pos)
	for i := 0; i < 26; i++ {
		if valid[i] == 2 {
			if pos[i] != distance[i] {
				return false
			}
		}
	}
	return true
}
