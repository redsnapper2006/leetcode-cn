package main

import "fmt"

func buildArray(target []int, n int) []string {
	var buf []int
	var ret []string
	cur := 1
	for {
		if len(buf) == 0 || buf[len(buf)-1] == target[len(buf)-1] {
			buf = append(buf, cur)
			cur++
			ret = append(ret, "Push")
		} else {
			buf = buf[0 : len(buf)-1]
			ret = append(ret, "Pop")

			buf = append(buf, cur)
			cur++
			ret = append(ret, "Push")
		}
		if len(buf) == len(target) && buf[len(buf)-1] == target[len(target)-1] {
			break
		}
	}
	return ret
}

func main() {
	fmt.Println(buildArray([]int{1, 3}, 3))
}
