package main

func decodeMessage(key string, message string) string {
	m := map[byte]byte{}
	o := [26]int{}

	idx := 0
	for _, k := range key {
		if k == ' ' {
			continue
		}
		if o[byte(k)-'a'] == 1 {
			continue
		}
		m[byte(k)] = byte('a' + idx)
		idx++
		o[byte(k)-'a'] = 1
	}

	r := []byte{}
	for _, b := range message {
		if b == ' ' {
			r = append(r, ' ')
			continue
		}
		r = append(r, m[byte(b)])
	}
	return string(r)
}
