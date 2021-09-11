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
	groupCnt := 0
	if scanner.Scan() {
		t, _ := strconv.ParseInt(scanner.Text(), 10, 32)
		groupCnt = int(t)
	}
	for i := 0; i < groupCnt; i++ {
		cnt := 0
		if scanner.Scan() {
			t, _ := strconv.ParseInt(scanner.Text(), 10, 32)
			cnt = int(t)
		}
		arr := ""
		if scanner.Scan() {
			arr = scanner.Text()
		}
		arrN := strings.Split(arr, " ")
		buf := []int{0}
		sum := 0
		for j := 0; j < cnt; j++ {
			n, _ := strconv.ParseInt(arrN[j], 10, 32)
			sum += int(n)
			buf = append(buf, sum)
		}
		if sum%2 == 1 {
			fmt.Println("NO")
			continue
		}
		half := sum / 2
		isOnceFound := false
		for i := len(buf) - 1; i >= 0 && buf[i] >= half; i-- {
			s, e := 0, i-1
			isFound := false
			for s <= e {
				m := s + (e-s)/2
				if buf[i]-buf[m] > half {
					s = m + 1
				} else if buf[i]-buf[m] < half {
					e = m - 1
				} else {
					isFound = true
					break
				}
			}
			if isFound {
				isOnceFound = true
				break
			}
		}
		if isOnceFound {
			fmt.Println("YES")
		} else {
			fmt.Println("NO")
		}
	}
}
