package main

import "fmt"

func getValidT9Words(num string, words []string) []string {
	M := map[byte]byte{
		'a': '2',
		'b': '2',
		'c': '2',
		'd': '3',
		'e': '3',
		'f': '3',
		'g': '4',
		'h': '4',
		'i': '4',
		'j': '5',
		'k': '5',
		'l': '5',
		'm': '6',
		'n': '6',
		'o': '6',
		'p': '7',
		'q': '7',
		'r': '7',
		's': '7',
		't': '8',
		'u': '8',
		'v': '8',
		'w': '9',
		'x': '9',
		'y': '9',
		'z': '9',
	}

	var ret []string
	for _, v := range words {
		var r []byte
		for _, b := range v {
			r = append(r, M[byte(b)])
		}
		if string(r) == num {
			ret = append(ret, v)
		}
	}
	return ret
}

func main() {
	fmt.Println()
}
