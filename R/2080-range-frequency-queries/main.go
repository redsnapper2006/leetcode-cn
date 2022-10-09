package main

type RangeFreqQuery struct {
	IM map[int][]int
}

func Constructor(arr []int) RangeFreqQuery {
	m := map[int][]int{}
	for i := 0; i < len(arr); i++ {
		m[arr[i]] = append(m[arr[i]], i)
	}

	return RangeFreqQuery{IM: m}
}

func (this *RangeFreqQuery) Query(left int, right int, value int) int {
	idx, ok := this.IM[value]
	if !ok {
		return 0
	}

	s, e := 0, len(idx)-1
	for s <= e {
		m := s + (e-s)/2
		if idx[m] < left {
			s = m + 1
		} else {
			e = m - 1
		}
	}
	ll := s

	s, e = 0, len(idx)-1
	for s <= e {
		m := s + (e-s)/2
		if idx[m] <= right {
			s = m + 1
		} else {
			e = m - 1
		}
	}
	lr := s
	return lr - ll
}
