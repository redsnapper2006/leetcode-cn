package main

import (
	"fmt"
	"strconv"
)

func getHint(secret string, guess string) string {
	A := 0
	aBuf := map[int]int{}
	bBuf := map[byte]int{}
	for i := 0; i < len(secret); i++ {
		if secret[i] == guess[i] {
			A++
			aBuf[i]++
		} else {
			bBuf[secret[i]]++
		}
	}

	B := 0
	for i := 0; i < len(guess); i++ {
		_, ok := aBuf[i]
		if ok {
			continue
		}
		_, ok2 := bBuf[guess[i]]
		if ok2 {
			B++
			bBuf[guess[i]]--
			if bBuf[guess[i]] == 0 {
				delete(bBuf, guess[i])
			}
		}
	}
	return strconv.FormatInt(int64(A), 10) + "A" + strconv.FormatInt(int64(B), 10) + "B"
}

func main() {
	fmt.Println("a")
}
