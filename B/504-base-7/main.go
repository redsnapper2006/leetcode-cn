package main

import "fmt"

func convertToBase7(num int) string {
	if num == 0 {
		return "0"
	}
	isMinus := false
	if num < 0 {
		isMinus = true
		num = -num
	}

	m := map[int]byte{
		0: '0',
		1: '1',
		2: '2',
		3: '3',
		4: '4',
		5: '5',
		6: '6',
	}
	var ret []byte
	for num > 0 {
		t := num % 7
		ret = append(ret, m[t])
		num /= 7
	}
	s, e := 0, len(ret)-1
	for s < e {
		ret[s], ret[e] = ret[e], ret[s]
		s++
		e--
	}
	if isMinus {
		return "-" + string(ret)
	}
	return string(ret)
}

func main() {
	fmt.Println("a")
}
