package main

import "fmt"

func reformatNumber(number string) string {
	var buf []byte
	for _, b := range number {
		if b == ' ' || b == '-' {
			continue
		}
		buf = append(buf, byte(b))
	}

	odd := len(buf) % 3
	end := len(buf)
	if odd == 2 {
		end = len(buf) - 2
	}
	if odd == 0 {
		end = len(buf) - 3
	}
	if odd == 1 {
		end = len(buf) - 4
	}
	var ret []byte
	for i := 0; i < end; i = i + 3 {
		ret = append(ret, buf[i], buf[i+1], buf[i+2], '-')
	}
	if odd == 2 {
		ret = append(ret, buf[len(buf)-2], buf[len(buf)-1])
	}
	if odd == 0 {
		ret = append(ret, buf[len(buf)-3], buf[len(buf)-2], buf[len(buf)-1])
	}
	if odd == 1 {
		ret = append(ret, buf[len(buf)-4], buf[len(buf)-3], '-', buf[len(buf)-2], buf[len(buf)-1])
	}
	return string(ret)
}

func main() {
	fmt.Println()
}
