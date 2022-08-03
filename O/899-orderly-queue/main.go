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
		ret := S
		for i := 1; i < len(S); i++ {
			buf := make([]byte, len(S))
			copy(buf, S[i:])
			copy(buf[len(S)-i:], S[0:i])
			if ret > string(buf) {
				ret = string(buf)
			}
		}
		return ret
	}

	buf := make([]byte, len(S))
	for i := 0; i < len(S); i++ {
		buf[i] = S[i]
	}
	sort.Sort(ByteSlice(buf))
	return string(buf)
}

func main() {
	// fmt.Println(orderlyQueue("cba", 1))
	// fmt.Println(orderlyQueue("baaca", 1))

	fmt.Println(orderlyQueue("gtxlrjwkpzolcynsrgqcbvphnoradctlfjrloykccsicuxcqtgvrlegvesooadiqjgjmpojlupzphnmwtwsghewxiamusracsvevypoakmylaobzrssykhcamttaqvwukssbbiqjqtuhzoqqrerlzszzvppmjkxqeallbfijqevmbcyaqerzxllhlyamxcdvhuhavilbqvfyqofwlbyjhbabwwmcdyoubbudvylcslnxodjwncnawgszxnisoxgsdkujhjjadtsqddmmdzvwervizcudgedrguuyuzoaikzkhuxbzszqarfzywsgyvqefopkvrgapixgofzqtxlolqivjuajmxstqxsqxtawetkkelzvtqfbyxaxtceegxkolmgighpaynnkttszkcusamvyjmltsmepajibculdyilseuvmsszujnknxcxndyfamobqoocjdmjiwqcrzjurmkfkgmrxdvtqebdihviezsumcplicihjsdjtiwweqqeomgsxxcjcmrsbcqvpccpfthvxnstqqkxeesfnxjtwrcnuzlbjmybxlctddgorwpqmnrhhqqefoviebnnporwiufimntatuaoadwbxrtrttxjjqnrjkkbtoxtkubqyxihhxikigwlnkikxhsfxmhltwvdzmyeyfwhvewrdylevbatctcydoqjcmixffplhdvcxydyguilsotkbixuypimmgwbizoyavqwtzitvsuvhhkaxvdvipmlpxkawyuektwwyqdkydyjpvccmxzreujgplnzawlvwtmnpkswiyoheshvyjjhgzvwayvvykbonftzsuuveppwlgnmabemrnozcjouwqrxupakbzvicojsvpvaglmveonqabckptqcwkvejrqnyvprknqvflefadtihdokbjutzwmuukkolqvxqnfkfsodeacqqnuogtjbflpuwutpfrsgjhzsjfigvejngeyxanblcufhcznkoeuhw", 1))
}
