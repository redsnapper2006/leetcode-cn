package main

import "fmt"

func canConstruct(ransomNote string, magazine string) bool {
	br, bm := map[byte]int{}, map[byte]int{}
	for i := 0; i < len(ransomNote); i++ {
		br[ransomNote[i]]++
	}
	for i := 0; i < len(magazine); i++ {
		bm[magazine[i]]++
	}

	for k, v := range br {
		vm, ok := bm[k]
		if !ok || vm < v {
			return false
		}
	}
	return true
}

func main() {
	fmt.Println("a")
}
