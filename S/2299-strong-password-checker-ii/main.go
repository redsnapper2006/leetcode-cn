package main

import "strings"

func strongPasswordCheckerII(password string) bool {
	hasLower, hasUpper, hasDigit, hasSpecial, hasContinuous := false, false, false, false, false
	for i, b := range password {
		if byte(b) >= 'a' && byte(b) <= 'z' {
			hasLower = true
		}
		if byte(b) >= 'A' && byte(b) <= 'Z' {
			hasUpper = true
		}
		if byte(b) >= '0' && byte(b) <= '9' {
			hasDigit = true
		}
		if strings.Contains("!@#$%^&*()-+", string(b)) {
			hasSpecial = true
		}
		if i < len(password)-1 && password[i] == password[i+1] {
			hasContinuous = true
		}
	}

	return len(password) >= 8 && hasLower && hasUpper && hasDigit && hasSpecial && !hasContinuous
}
