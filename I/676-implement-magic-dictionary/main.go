package main

type MagicDictionary struct {
	M map[int][]string
}

func Constructor() MagicDictionary {
	return MagicDictionary{M: map[int][]string{}}
}

func (this *MagicDictionary) BuildDict(dictionary []string) {
	for _, word := range dictionary {
		l := len(word)
		_, ok := this.M[l]
		if !ok {
			this.M[l] = []string{}
		}

		this.M[l] = append(this.M[l], word)
	}
}

func (this *MagicDictionary) Search(searchWord string) bool {
	l := len(searchWord)
	candidate, ok := this.M[l]
	if !ok {
		return false
	}

	for _, candi := range candidate {
		unmatched := 0
		for i := 0; i < len(candi); i++ {
			if candi[i] == searchWord[i] {
				continue
			}
			unmatched++
			if unmatched > 1 {
				break
			}
		}
		if unmatched == 1 {
			return true
		}
	}
	return false
}
