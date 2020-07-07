package main

import (
	"fmt"
	"sort"
	"strconv"
)

func findMinDifference(timePoints []string) int {
	sort.Strings(timePoints)
	ret := 24 * 60
	for i := 1; i < len(timePoints); i++ {
		a, b := timePoints[i-1], timePoints[i]
		ma, _ := strconv.ParseInt(a[3:], 10, 32)
		mb, _ := strconv.ParseInt(b[3:], 10, 32)
		ha, _ := strconv.ParseInt(a[0:2], 10, 32)
		hb, _ := strconv.ParseInt(b[0:2], 10, 32)
		hdiff := hb - ha
		if mb < ma {
			mb += 60
			hdiff--
		}
		mdiff := mb - ma
		if ret > int(hdiff)*60+int(mdiff) {
			ret = int(hdiff)*60 + int(mdiff)
		}
	}
	a, b := timePoints[len(timePoints)-1], timePoints[0]
	ma, _ := strconv.ParseInt(a[3:], 10, 32)
	mb, _ := strconv.ParseInt(b[3:], 10, 32)
	ha, _ := strconv.ParseInt(a[0:2], 10, 32)
	hb, _ := strconv.ParseInt(b[0:2], 10, 32)
	hdiff := hb + 24 - ha
	if mb < ma {
		mb += 60
		hdiff--
	}
	mdiff := mb - ma
	if ret > int(hdiff)*60+int(mdiff) {
		ret = int(hdiff)*60 + int(mdiff)
	}

	return ret
}

func main() {
	fmt.Println(findMinDifference([]string{"23:59", "00:00"}))
}
