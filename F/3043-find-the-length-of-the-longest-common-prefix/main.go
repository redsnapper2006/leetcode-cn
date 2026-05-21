package main

type Pre struct {
	Child []*Pre
}

func longestCommonPrefix(arr1 []int, arr2 []int) int {
	var build func(d []int, idx int, pre *Pre)
	build = func(d []int, idx int, pre *Pre) {
		if idx == -1 {
			return
		}

		if pre.Child[d[idx]] == nil {
			pre.Child[d[idx]] = &Pre{Child: make([]*Pre, 10)}
		}
		build(d, idx-1, pre.Child[d[idx]])
	}

	var search func(d []int, idx int, step int, pre *Pre, ans *int)
	search = func(d []int, idx int, step int, pre *Pre, ans *int) {
		if idx == -1 || pre.Child[d[idx]] == nil {
			if step > *ans {
				*ans = step
			}
		} else {
			search(d, idx-1, step+1, pre.Child[d[idx]], ans)
		}
	}

	root := &Pre{Child: make([]*Pre, 10)}
	for _, v := range arr1 {
		d := []int{}
		vv := v
		for vv > 0 {
			d = append(d, vv%10)
			vv /= 10
		}

		build(d, len(d)-1, root)
	}

	ans := 0
	for _, v := range arr2 {
		d := []int{}
		vv := v
		for vv > 0 {
			d = append(d, vv%10)
			vv /= 10
		}

		search(d, len(d)-1, 0, root, &ans)
	}
	return ans

}
