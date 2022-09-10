package main

func findKthBit(n int, k int) byte {
	buf := []byte{'0'}

	for i := 0; i < n; i++ {
		s := len(buf)
		buf = append(buf, '1')
		for j := s - 1; j >= 0; j-- {
			b := byte('0')
			if buf[j] == '0' {
				b = byte('1')
			}
			buf = append(buf, b)
		}
	}
	return buf[k-1]
}
