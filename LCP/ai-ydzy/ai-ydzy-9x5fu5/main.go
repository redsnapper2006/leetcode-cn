package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

func main() {
	scanner := bufio.NewScanner(os.Stdin)
	colCnt, blockCnt := 0, 0
	if scanner.Scan() {
		t := scanner.Text()
		arr := strings.Split(t, " ")
		tt1, _ := strconv.ParseInt(arr[0], 10, 32)
		colCnt = int(tt1)
		tt2, _ := strconv.ParseInt(arr[1], 10, 32)
		blockCnt = int(tt2)
	}
	if scanner.Scan() {
		line := scanner.Text()
		arr := strings.Split(line, " ")
		buf := make([]int, colCnt)
		for i := 0; i < blockCnt; i++ {
			t, _ := strconv.ParseInt(arr[i], 10, 32)
			idx := int(t) - 1
			buf[idx]++
		}
		min := 1<<31 - 1
		for i := 0; i < colCnt; i++ {
			if min > buf[i] {
				min = buf[i]
			}
		}
		fmt.Println(min)
	}
}
