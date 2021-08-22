package main

import "fmt"

type Trie struct {
	ZeroCnt int
	Zero    *Trie
	OneCnt  int
	One     *Trie
}

func findDifferentBinaryString(nums []string) string {
	root := Trie{}

	for _, n := range nums {
		p := &root
		for _, b := range n {
			zeroOrOne := int(b - '0')

			if zeroOrOne == 0 {
				p.ZeroCnt++
				// fmt.Println("build", zeroOrOne, p)
				if p.Zero == nil {
					p.Zero = &Trie{}
				}
				p = p.Zero
			} else {
				p.OneCnt++
				// fmt.Println("build", zeroOrOne, p)
				if p.One == nil {
					p.One = &Trie{}
				}
				p = p.One
			}
			if p == nil {
				p = &Trie{}
			}
		}
	}
	ret := []byte{}
	p := &root
	for p != nil {
		// fmt.Println("cur", p)
		if p.OneCnt > p.ZeroCnt {
			// fmt.Println("goto zero")
			ret = append(ret, '0')
			p = p.Zero
		} else {
			// fmt.Println("goto one")
			ret = append(ret, '1')
			p = p.One
			// fmt.Println(p)
		}
	}
	if len(ret) < len(nums[0]) {
		for i := len(ret); i < len(nums[0]); i++ {
			ret = append(ret, '0')
		}
	}
	return string(ret)
}

func main() {
	fmt.Println(findDifferentBinaryString([]string{"01", "10"}))
}
