package main

import (
	"fmt"
	"strconv"
	"strings"
)

func subdomainVisits(cpdomains []string) []string {
	M := map[string]int64{}
	for i := 0; i < len(cpdomains); i++ {
		d := cpdomains[i]
		kv := strings.Split(d, " ")
		c := kv[0]
		domain := kv[1]
		v, _ := strconv.ParseInt(c, 10, 32)
		arr := strings.Split(domain, ".")
		for j := 0; j < len(arr); j++ {
			M[strings.Join(arr[j:], ".")] += v
		}
	}
	var buf []string
	for k, v := range M {
		buf = append(buf, strconv.FormatInt(v, 10)+" "+k)
	}
	return buf
}

func main() {
	fmt.Println("a")
}
