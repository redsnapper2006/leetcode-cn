package main

import (
	"fmt"
	"strings"
)

func reformatDate(date string) string {
	arr := strings.Split(date, " ")
	d := []byte{}
	for i := 0; i < len(arr[0]); i++ {
		if arr[0][i] <= '9' && arr[0][i] >= '0' {
			d = append(d, arr[0][i])
		} else {
			break
		}
	}

	if len(d) == 1 {
		d = append([]byte{'0'}, d...)
	}

	M := map[string]string{"Jan": "01", "Feb": "02", "Mar": "03", "Apr": "04", "May": "05", "Jun": "06", "Jul": "07", "Aug": "08", "Sep": "09", "Oct": "10", "Nov": "11", "Dec": "12"}
	m := M[arr[1]]

	return arr[2] + "-" + m + "-" + string(d)

}

func main() {
	fmt.Println("a")
}
