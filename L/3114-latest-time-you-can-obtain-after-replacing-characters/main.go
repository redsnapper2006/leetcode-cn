
func findLatestTime(s string) string {
	ans := []byte(s)
	if ans[0] == '?' {
		if ans[1] != '?' && ans[1] > '1' {
			ans[0] = '0'
		} else {
			ans[0] = '1'
		}
	}
	if ans[1] == '?' {
		if ans[0] == '1' {
			ans[1] = '1'
		} else {
			ans[1] = '9'
		}
	}
	if ans[3] == '?' {
		ans[3] = '5'
	}
	if ans[4] == '?' {
		ans[4] = '9'
	}
	return string(ans)
}
