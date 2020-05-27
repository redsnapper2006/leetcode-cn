package main

import "fmt"

func camelMatch(queries []string, pattern string) []bool {
	split := func(str string) []string {
		var P []string
		sIdx := -1
		for i := 0; i < len(str); i++ {
			if str[i] <= 'Z' && str[i] >= 'A' {
				if sIdx != -1 {
					P = append(P, str[sIdx:i])
				}
				sIdx = i
			}
		}
		if sIdx == -1 {
			return nil
		}
		P = append(P, str[sIdx:])
		return P
	}

	P := split(pattern)

	var ret []bool
	for i := 0; i < len(queries); i++ {
		Q := split(queries[i])
		if len(P) != len(Q) {
			ret = append(ret, false)
		} else {
			isMatch := true
			for j := 0; j < len(P); j++ {
				sp, sq := 0, 0
				for sp < len(P[j]) && sq < len(Q[j]) {
					if P[j][sp] == Q[j][sq] {
						sp++
						sq++
					} else {
						sq++
					}
				}

				if sp != len(P[j]) {
					isMatch = false
					break
				}
			}

			ret = append(ret, isMatch)
		}
	}
	return ret
}

func main() {
	fmt.Println(camelMatch([]string{"FooBar", "FooBarTest", "FootBall", "FrameBuffer", "ForceFeedBack"}, "FB"))
	fmt.Println(camelMatch([]string{"CompetitiveProgramming", "CounterPick", "ControlPanel"}, "CooP"))
}
