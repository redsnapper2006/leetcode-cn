package main

import "fmt"

// zero
// six
// two
// four
// eight

// one
// three
// five
// seven

// nine

func originalDigits(s string) string {
	M := map[byte]int{}
	for i := 0; i < len(s); i++ {
		M[s[i]]++
	}

	order := []byte{'z', 'x', 'w', 'u', 'g', 'o', 't', 'f', 's', 'i'}
	number := []byte{'0', '6', '2', '4', '8', '1', '3', '5', '7', '9'}
	erase := [][]byte{{'z', 'e', 'r', 'o'},
		{'s', 'i', 'x'},
		{'t', 'w', 'o'},
		{'f', 'o', 'u', 'r'},
		{'e', 'i', 'g', 'h', 't'},
		{'o', 'n', 'e'},
		{'t', 'h', 'r', 'e', 'e'},
		{'f', 'i', 'v', 'e'},
		{'s', 'e', 'v', 'e', 'n'},
		{'n', 'i', 'n', 'e'},
	}
	var buf [][]byte

	for i := 0; i < len(order); i++ {
		c := M[order[i]]
		var b []byte
		for j := 0; j < c; j++ {
			b = append(b, number[i])
		}
		for j := 0; j < len(erase[i]); j++ {
			M[erase[i][j]] -= c
		}
		buf = append(buf, b)
	}

	return string(buf[0]) + string(buf[5]) + string(buf[2]) + string(buf[6]) + string(buf[3]) + string(buf[7]) + string(buf[1]) + string(buf[8]) + string(buf[4]) + string(buf[9])
}

func main() {
	fmt.Println("a")
}
