package main

import (
	"bufio"
	"fmt"
	"os"
)

func main() {
	scanner := bufio.NewScanner(os.Stdin)

	scanner.Scan()
	l1 := scanner.Text()
	scanner.Scan()
	l2 := scanner.Text()
	scanner.Scan()
	l3 := scanner.Text()
	scanner.Scan()
	l4 := scanner.Text()
	lines := []string{l1, l2, l3, l4}

	var ret []byte
	for i := 0; i < len(l1); i++ {
		M := map[byte]int{}
		for j := 0; j < 4; j++ {
			M[lines[j][i]]++
		}
		var b byte
		cnt := -1
		for k, v := range M {
			if v > cnt {
				cnt = v
				b = k
			}
		}
		isFirst := true
		for _, v := range M {
			if v == cnt {
				if isFirst {
					isFirst = false
				} else {
					fmt.Println("Input Error")
					os.Exit(0)
				}
			}
		}
		ret = append(ret, b)
	}
	fmt.Println(string(ret))

}
