package main

func maximumLength(s string) int {
	buf := make([][]int, 26)
	for i := 0; i < 26; i++ {
		buf[i] = make([]int, 3)
	}

	base := byte(' ')
	cnt := 0
	for _, b := range []byte(s) {
		if base != b {
			if base != ' ' && cnt != 0 {
				offset := int(base - 'a')
				if buf[offset][0] <= cnt {
					buf[offset][2] = buf[offset][1]
					buf[offset][1] = buf[offset][0]
					buf[offset][0] = cnt
				} else if buf[offset][1] <= cnt {
					buf[offset][2] = buf[offset][1]
					buf[offset][1] = cnt
				} else if buf[offset][2] <= cnt {
					buf[offset][2] = cnt
				}
			}
			base = b
			cnt = 1
		} else {
			cnt++
		}
	}
	if base != ' ' && cnt != 0 {
		offset := int(base - 'a')
		if buf[offset][0] <= cnt {
			buf[offset][2] = buf[offset][1]
			buf[offset][1] = buf[offset][0]
			buf[offset][0] = cnt
		} else if buf[offset][1] <= cnt {
			buf[offset][2] = buf[offset][1]
			buf[offset][1] = cnt
		} else if buf[offset][2] <= cnt {
			buf[offset][2] = cnt
		}
	}

	max := -1
	for i := 0; i < 26; i++ {
		if buf[i][0] == 0 {
			continue
		}
		ans := 0
		// fmt.Println(buf[i])
		if buf[i][0] == buf[i][1] && buf[i][1] == buf[i][2] {
			ans = buf[i][0]
		} else if (buf[i][0] == buf[i][1] && buf[i][1] > 1) || (buf[i][0] == buf[i][1]+1 && buf[i][1] > 0) {
			ans = buf[i][0] - 1
		} else if buf[i][0] > 2 {
			ans = buf[i][0] - 2
		} else {
			ans = -1
		}
		if max < ans {
			max = ans
		}
	}

	return max
}
