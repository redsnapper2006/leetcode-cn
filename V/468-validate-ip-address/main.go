package main

import (
	"fmt"
	"strings"
)

func validIPAddress(IP string) string {
	ipv4 := func(IP string) bool {
		arr := strings.Split(IP, ".")
		if len(arr) != 4 {
			return false
		}
		for i := 0; i < 4; i++ {
			if len(arr[i]) == 0 || len(arr[i]) > 1 && arr[i][0] == '0' {
				return false
			}
			c := 0
			for _, b := range arr[i] {
				if b > '9' || b < '0' {
					return false
				}
				c = c*10 + int(b-'0')
			}
			if c > 255 {
				return false
			}
		}
		return true
	}

	ipv6 := func(IP string) bool {
		arr := strings.Split(IP, ":")
		if len(arr) != 8 {
			return false
		}
		for i := 0; i < 8; i++ {
			if len(arr[i]) < 1 || len(arr[i]) > 4 {
				return false
			}
			for _, b := range arr[i] {
				if !((b <= '9' && b >= '0') || (b <= 'f' && b >= 'a') || (b <= 'F' && b >= 'A')) {
					return false
				}
			}
		}
		return true
	}
	v4 := ipv4(IP)
	v6 := ipv6(IP)
	if !v4 && !v6 {
		return "Neither"
	}
	if v4 {
		return "IPv4"
	}
	return "IPv6"
}

func main() {
	fmt.Println()
}
