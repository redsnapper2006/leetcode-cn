package main

type StreamChecker struct {
	Buf      []byte
	Children [26]*StreamChecker
	IsEnd    bool
}

func Constructor(words []string) StreamChecker {
	root := StreamChecker{Buf: []byte{}, Children: [26]*StreamChecker{}}
	for _, word := range words {
		p := &root
		for i := len(word) - 1; i >= 0; i-- {
			offset := int(word[i] - 'a')
			if p.Children[offset] == nil {
				p.Children[offset] = &StreamChecker{Children: [26]*StreamChecker{}}
			}
			p = p.Children[offset]
		}
		p.IsEnd = true
	}
	return root
}

func (this *StreamChecker) Query(letter byte) bool {
	this.Buf = append(this.Buf, letter)
	p := this
	for i := len(this.Buf) - 1; i >= 0; i-- {
		offset := int(this.Buf[i] - 'a')
		if p.IsEnd {
			return true
		}
		if p.Children[offset] == nil {
			return false
		}
		p = p.Children[offset]
	}
	return p.IsEnd
}
