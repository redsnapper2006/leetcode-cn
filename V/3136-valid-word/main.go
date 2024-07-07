func isValid(word string) bool {
	if len(word) < 3 {
		return false
	}

	hasVowel, hasConsonant := false, false
	for _, b := range []byte(word) {
		if (b < '0' || b > '9') && (b < 'a' || b > 'z') && (b < 'A' || b > 'Z') {
			return false
		}
		if b == 'A' || b == 'a' || b == 'E' || b == 'e' || b == 'I' || b == 'i' || b == 'O' || b == 'o' || b == 'U' || b == 'u' {
			hasVowel = true
		} else if b <= '0' || b >= '9' {
			hasConsonant = true
		}
	}
	return hasVowel && hasConsonant
}
