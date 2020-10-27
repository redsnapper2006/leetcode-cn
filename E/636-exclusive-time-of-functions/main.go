package main

import (
	"fmt"
	"strconv"
	"strings"
)

func exclusiveTime(n int, logs []string) []int {
	M := map[int]int{}

	start := logs[0]
	arr := strings.Split(start, ":")
	t, _ := strconv.ParseInt(arr[0], 10, 32)
	tt, _ := strconv.ParseInt(arr[2], 10, 32)
	baseT := int(tt)
	stack := []int{int(t)}

	for i := 1; i < len(logs); i++ {
		arr := strings.Split(logs[i], ":")
		tfid, _ := strconv.ParseInt(arr[0], 10, 32)
		tt, _ := strconv.ParseInt(arr[2], 10, 32)

		if arr[1] == "start" {
			if len(stack) > 0 {
				M[stack[len(stack)-1]] += int(tt) - baseT - 1
			}
			stack = append(stack, int(tfid))
		} else {
			stack = stack[0 : len(stack)-1]
			M[int(tfid)] += int(tt) - baseT + 1
		}
		baseT = int(tt)
	}

	var ret []int
	for i := 0; i < n; i++ {
		v, ok := M[i]
		t := 0
		if ok {
			t = v
		}
		ret = append(ret, t)
	}
	return ret
}

func main() {
	fmt.Println("a")
}
