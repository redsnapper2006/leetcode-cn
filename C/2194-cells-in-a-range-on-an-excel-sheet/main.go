package main

func cellsInRange(s string) []string {
	ret := []string{}
	for i := int(s[0] - 'A'); i <= int(s[3]-'A'); i++ {
		for j := int(s[1] - '0'); j <= int(s[4]-'0'); j++ {
			ret = append(ret, string([]byte{byte('A' + i), byte('0' + j)}))
		}
	}
	return ret
}
