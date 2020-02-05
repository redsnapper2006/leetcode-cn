package main

import (
	"fmt"
)

func reverseBits(num uint32) uint32 {
	var r uint32
	for i := 0; i < 32; i++ {
		r = r << 1
		if num&(1<<i) >= 1 {
			r++
		}
	}
	return r
}

func main() {
	fmt.Printf("%d\n", reverseBits(43261596))
	fmt.Printf("%v\n", reverseBits(4294967293))
}
