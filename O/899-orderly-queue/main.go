package main

import (
	"fmt"
	"sort"
)

type ByteSlice []byte

func (p ByteSlice) Len() int {
	return len(p)
}

func (p ByteSlice) Swap(i, j int) {
	p[i], p[j] = p[j], p[i]
}

func (p ByteSlice) Less(i, j int) bool {
	return p[i] < p[j]
}

func orderlyQueue(S string, K int) string {
	if K == 1 {
		min := int('z') + 1
		idxArr := []int{}
		for i := 0; i < len(S); i++ {
			if min > int(S[i]) {
				min = int(S[i])
				idxArr = []int{i}
			} else if min == int(S[i]) {
				idxArr = append(idxArr, i)
			}
		}
		if len(idxArr) == 1 {
			buf := make([]byte, len(S))
			copy(buf, S[idxArr[0]:])
			copy(buf[(len(S)-idxArr[0]):], S[0:idxArr[0]])
			return string(buf)
		}

		for i := 0; i < len(S); i++ {
			// fmt.Println(idxArr)
			// fmt.Println("loop", i)
			min := S[(idxArr[0]+i)%len(S)]
			isFound := false
			t := []int{idxArr[0]}
			for j := 1; j < len(idxArr); j++ {
				if S[(idxArr[j]+i)%len(S)] != min {
					isFound = true
				}
				// fmt.Println(string(min), string(S[(idxArr[j]+i)%len(S)]), j)
				if min > S[(idxArr[j]+i)%len(S)] {
					min = S[(idxArr[j]+i)%len(S)]
					t = []int{idxArr[j]}
				} else if min == S[(idxArr[j]+i)%len(S)] {
					t = append(t, idxArr[j])
				}
			}
			if isFound {
				if len(t) == 1 {
					buf := make([]byte, len(S))
					copy(buf, S[t[0]:])
					copy(buf[(len(S)-t[0]):], S[0:t[0]])
					return string(buf)
				} else {
					idxArr = t
				}
			}
		}
	} else {
		buf := make([]byte, len(S))
		for i := 0; i < len(S); i++ {
			buf[i] = S[i]
		}
		sort.Sort(ByteSlice(buf))
		return string(buf)
	}
	return ""
}

func main() {
	// fmt.Println(orderlyQueue("cba", 1))
	// fmt.Println(orderlyQueue("baaca", 1))

	fmt.Println(orderlyQueue("gtxlrjwkpzolcynsrgqcbvphnoradctlfjrloykccsicuxcqtgvrlegvesooadiqjgjmpojlupzphnmwtwsghewxiamusracsvevypoakmylaobzrssykhcamttaqvwukssbbiqjqtuhzoqqrerlzszzvppmjkxqeallbfijqevmbcyaqerzxllhlyamxcdvhuhavilbqvfyqofwlbyjhbabwwmcdyoubbudvylcslnxodjwncnawgszxnisoxgsdkujhjjadtsqddmmdzvwervizcudgedrguuyuzoaikzkhuxbzszqarfzywsgyvqefopkvrgapixgofzqtxlolqivjuajmxstqxsqxtawetkkelzvtqfbyxaxtceegxkolmgighpaynnkttszkcusamvyjmltsmepajibculdyilseuvmsszujnknxcxndyfamobqoocjdmjiwqcrzjurmkfkgmrxdvtqebdihviezsumcplicihjsdjtiwweqqeomgsxxcjcmrsbcqvpccpfthvxnstqqkxeesfnxjtwrcnuzlbjmybxlctddgorwpqmnrhhqqefoviebnnporwiufimntatuaoadwbxrtrttxjjqnrjkkbtoxtkubqyxihhxikigwlnkikxhsfxmhltwvdzmyeyfwhvewrdylevbatctcydoqjcmixffplhdvcxydyguilsotkbixuypimmgwbizoyavqwtzitvsuvhhkaxvdvipmlpxkawyuektwwyqdkydyjpvccmxzreujgplnzawlvwtmnpkswiyoheshvyjjhgzvwayvvykbonftzsuuveppwlgnmabemrnozcjouwqrxupakbzvicojsvpvaglmveonqabckptqcwkvejrqnyvprknqvflefadtihdokbjutzwmuukkolqvxqnfkfsodeacqqnuogtjbflpuwutpfrsgjhzsjfigvejngeyxanblcufhcznkoeuhw", 1))
}
