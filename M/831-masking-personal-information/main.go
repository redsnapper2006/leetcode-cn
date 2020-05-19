package main

import (
	"fmt"
	"strings"
)

func maskPII(S string) string {
	atIdx := strings.Index(S, "@")

	if atIdx != -1 {
		arr := strings.Split(S, "@")
		name := strings.ToLower(arr[0])
		remained := strings.ToLower(arr[1])
		return string(name[0]) + "*****" + string(name[len(name)-1]) + "@" + remained
	} else {
		var buf []byte
		for i := 0; i < len(S); i++ {
			if S[i] <= '9' && S[i] >= '0' {
				buf = append(buf, S[i])
			}
		}
		if len(buf) == 10 {
			return "***-***-" + string(buf[6:])
		} else {
			prefix := "+"
			for i := 0; i < len(buf)-10; i++ {
				prefix += "*"
			}
			return prefix + "-***-***-" + string(buf[len(buf)-4:])
		}
	}
}

func main() {
	fmt.Println("a")
}
