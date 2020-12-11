package main

import "fmt"

func predictPartyVictory(senate string) string {
	t := []byte(senate)
	DC, RC := 0, 0
	for len(t) > 0 {
		var buf []byte

		for _, v := range t {
			if v == 'R' {
				if DC > 0 {
					DC--
				} else {
					buf = append(buf, 'R')
					RC++
				}
			} else {
				if RC > 0 {
					RC--
				} else {
					buf = append(buf, 'D')
					DC++
				}
			}
		}
		b := buf[0]
		isOnly := true
		for i := 1; i < len(buf); i++ {
			if buf[i] != b {
				isOnly = false
				break
			}
		}
		if isOnly {
			if b == 'D' {
				return "Dire"
			}
			if b == 'R' {
				return "Radiant"
			}
		}

		t = buf
	}
	return ""
}

func main() {
	fmt.Println(predictPartyVictory("DRRDRDRDRDDRDRDR"))
}
