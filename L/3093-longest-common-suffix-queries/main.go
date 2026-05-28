type Trie struct {
	Child []*Trie
	Len   int
	Idx   int
}

func stringIndices(wordsContainer []string, wordsQuery []string) []int {
	const MAX_LEN int = 5100
	var build func(root *Trie, bb []byte, bbIdx int, arrIdx int)
	build = func(root *Trie, bb []byte, bbIdx int, arrIdx int) {

		if root.Len > len(bb) {
			root.Len = len(bb)
			root.Idx = arrIdx
		} else if root.Len == len(bb) && root.Idx > arrIdx {
			root.Idx = arrIdx
		}
		if bbIdx == -1 {
			return
		}

		offset := bb[bbIdx] - 'a'
		if root.Child[offset] == nil {
			root.Child[offset] = &Trie{Child: make([]*Trie, 26), Len: MAX_LEN}
		}
		build(root.Child[offset], bb, bbIdx-1, arrIdx)
	}

	root := &Trie{Child: make([]*Trie, 26), Len: MAX_LEN}
	for arrIdx, wc := range wordsContainer {
		bb := []byte(wc)
		build(root, bb, len(bb)-1, arrIdx)
	}

	var search func(root *Trie, bb []byte, bbIdx int) int
	search = func(root *Trie, bb []byte, bbIdx int) int {
		if bbIdx == -1 || root.Child[bb[bbIdx]-'a'] == nil {
			return root.Idx
		}

		return search(root.Child[bb[bbIdx]-'a'], bb, bbIdx-1)
	}

	ans := []int{}
	for _, wq := range wordsQuery {
		bb := []byte(wq)
		r := search(root, bb, len(bb)-1)
		ans = append(ans, r)
	}
	return ans
}
