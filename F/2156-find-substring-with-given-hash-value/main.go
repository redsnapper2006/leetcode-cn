package main

func subStrHash(s string, power int, modulo int, k int, hashValue int) string {
	letters, times := make([]int, len(s)), make([]int, k+1)
	for i := 0; i < len(s); i++ {
		letters[i] = int(byte(s[i]-'a'+1)) % modulo
	}
	t := 1
	times[0] = 1
	for i := 1; i < k+1; i++ {
		t *= power
		t %= modulo
		times[i] = t
	}
	sum := 0
	for i := 0; i < k; i++ {
		sum += letters[len(s)-k+i] * times[i]
		sum %= modulo
	}

	ret := -1
	if sum == hashValue {
		ret = len(s) - k
	}

	for i := len(s) - k - 1; i >= 0; i-- {
		sum = (sum*power + letters[i] + modulo - letters[i+k]*times[len(times)-1]%modulo) % modulo
		// fmt.Println(sum, hashValue, i)
		if sum == hashValue {
			ret = i
		}
	}

	return s[ret : ret+k]
}

func subStrHash2(s string, power int, modulo int, k int, hashValue int) string {
	letters, times := make([]int, len(s)), make([]int, k)
	for i := 0; i < len(s); i++ {
		letters[i] = int(byte(s[i]-'a'+1)) % modulo
	}
	t := 1
	times[0] = 1
	for i := 1; i < k; i++ {
		t *= power
		t %= modulo
		times[i] = t
	}
	for i := 0; i < len(s)-k; i++ {
		sum := 0
		for j := 0; j < k; j++ {
			sum += letters[i+j] * times[j]
			sum %= modulo
		}
		if sum%modulo == hashValue {
			return s[i : i+k]
		}
	}

	return ""
}
