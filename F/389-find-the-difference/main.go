package main

import "fmt"

func findTheDifference(s string, t string) byte {
	bs, bt := map[byte]int{}, map[byte]int{}
	for i := 0; i < len(s); i++ {
		bs[s[i]]++
	}
	for i := 0; i < len(t); i++ {
		bt[t[i]]++
	}
	for k, v := range bs {
		bt[k] -= v
		if bt[k] == 0 {
			delete(bt, k)
		}
	}
	for k := range bt {
		return k
	}
	return '0'
}

func main() {
	fmt.Println("a")
}
