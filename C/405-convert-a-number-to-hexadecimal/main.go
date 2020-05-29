package main

import "fmt"

func toHex(num int) string {
	if num == 0 {
		return "0"
	}
	M := map[int]byte{
		0:  '0',
		1:  '1',
		2:  '2',
		3:  '3',
		4:  '4',
		5:  '5',
		6:  '6',
		7:  '7',
		8:  '8',
		9:  '9',
		10: 'a',
		11: 'b',
		12: 'c',
		13: 'd',
		14: 'e',
		15: 'f',
	}
	if num < 0 {
		num = -num
		num = ^num
		num++
	}
	var r []byte
	for i := 0; i < 8; i++ {
		t := num >> (4 * i) & 0xF
		r = append(r, M[t])
	}
	s, e := 0, 7
	for s < e {
		r[s], r[e] = r[e], r[s]
		s++
		e--
	}

	idx := -1
	for i := 0; i < len(r); i++ {
		if r[i] != '0' {
			idx = i
			break
		}
	}

	return string(r[idx:])
}

func main() {

	fmt.Println(toHex(26))
	fmt.Println(toHex(-1))
}
