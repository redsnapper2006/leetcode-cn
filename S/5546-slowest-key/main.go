package main

import "fmt"

func slowestKey(releaseTimes []int, keysPressed string) byte {
	buf := make([]int, 26)
	s := 0
	for i, v := range releaseTimes {
		if v-s > buf[keysPressed[i]-'a'] {
			buf[keysPressed[i]-'a'] = v - s
		}

		s = v
	}
	max := -1
	var b byte
	for i := 0; i < 26; i++ {
		if buf[i] >= max {
			max = buf[i]
			b = byte('a' + i)
		}
	}
	return b
}

func main() {
	fmt.Println("a")
}
