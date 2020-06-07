package main

import (
	"fmt"
	"sort"
)

func findLadders(beginWord string, endWord string, wordList []string) [][]string {
	isSimilar := func(s, t string) bool {
		if len(s) != len(t) {
			return false
		}
		c := 0
		for i := 0; i < len(s); i++ {
			if s[i] != t[i] {
				c++
			}
		}
		return c == 1
	}

	MAX := len(wordList) + 2

	M := make(map[string][]string)
	E := make(map[string]int)
	wordList = append(wordList, beginWord)
	sort.Strings(wordList)
	var candi []string
	for i := 0; i < len(wordList); i++ {
		if i > 0 && wordList[i] == wordList[i-1] {
			continue
		}
		candi = append(candi, wordList[i])
	}

	for i := 0; i < len(candi); i++ {
		for j := i + 1; j < len(candi); j++ {
			if isSimilar(candi[i], candi[j]) {
				v, ok := M[candi[i]]
				if !ok {
					M[candi[i]] = []string{candi[j]}
				} else {
					M[candi[i]] = append(v, candi[j])
				}

				v, ok = M[candi[j]]
				if !ok {
					M[candi[j]] = []string{candi[i]}
				} else {
					M[candi[j]] = append(v, candi[i])
				}
			}
		}
		E[candi[i]] = MAX
	}

	_, ok := M[endWord]
	if !ok {
		return [][]string{}
	}

	buf := [][]string{{beginWord}}
	step := 1
	for {
		step++
		var bb [][]string
		for i := 0; i < len(buf); i++ {
			p := buf[i]
			t := M[p[len(p)-1]]
			for j := 0; j < len(t); j++ {
				if E[t[j]] == MAX {
					var copy []string
					for m := 0; m < len(p); m++ {
						copy = append(copy, p[m])
					}
					copy = append(copy, t[j])
					bb = append(bb, copy)
				}
			}
		}
		if len(bb) == 0 {
			break
		}
		for i := 0; i < len(bb); i++ {
			p := bb[i]
			E[p[len(p)-1]] = step
		}

		if E[endWord] < MAX {
			var ret [][]string
			for i := 0; i < len(bb); i++ {
				c := bb[i]
				if c[len(c)-1] == endWord {
					ret = append(ret, c)
				}
			}
			return ret
		}
		buf = bb
	}

	if E[endWord] < MAX {
		var ret [][]string
		for i := 0; i < len(buf); i++ {
			c := buf[i]
			if c[len(c)-1] == endWord {
				ret = append(ret, c)
			}
		}
		return ret
	}

	return [][]string{}
}

func main() {
	// fmt.Println(findLadders("hit", "cog", []string{"hot", "dot", "dog", "lot", "log", "cog"}))
	// fmt.Println(findLadders("red", "tax", []string{"ted", "tex", "red", "tax", "tad", "den", "rex", "pee"}))

	fmt.Println(findLadders("cet", "ism",
		[]string{"kid", "tag", "pup", "ail", "tun", "woo", "erg", "luz", "brr", "gay", "sip", "kay", "per", "val", "mes", "ohs", "now", "boa", "cet", "pal", "bar", "die", "war", "hay", "eco", "pub", "lob", "rue", "fry", "lit", "rex", "jan", "cot", "bid", "ali", "pay", "col", "gum", "ger", "row", "won", "dan", "rum", "fad", "tut", "sag", "yip", "sui", "ark", "has", "zip", "fez", "own", "ump", "dis", "ads", "max", "jaw", "out", "btu", "ana", "gap", "cry", "led", "abe", "box", "ore", "pig", "fie", "toy", "fat", "cal", "lie", "noh", "sew", "ono", "tam", "flu", "mgm", "ply", "awe", "pry", "tit", "tie", "yet", "too", "tax", "jim", "san", "pan", "map", "ski", "ova", "wed", "non", "wac", "nut", "why", "bye", "lye", "oct", "old", "fin", "feb", "chi", "sap", "owl", "log", "tod", "dot", "bow", "fob", "for", "joe", "ivy", "fan", "age", "fax", "hip", "jib", "mel", "hus", "sob", "ifs", "tab", "ara", "dab", "jag", "jar", "arm", "lot", "tom", "sax", "tex", "yum", "pei", "wen", "wry", "ire", "irk", "far", "mew", "wit", "doe", "gas", "rte", "ian", "pot", "ask", "wag", "hag", "amy", "nag", "ron", "soy", "gin", "don", "tug", "fay", "vic", "boo", "nam", "ave", "buy", "sop", "but", "orb", "fen", "paw", "his", "sub", "bob", "yea", "oft", "inn", "rod", "yam", "pew", "web", "hod", "hun", "gyp", "wei", "wis", "rob", "gad", "pie", "mon", "dog", "bib", "rub", "ere", "dig", "era", "cat", "fox", "bee", "mod", "day", "apr", "vie", "nev", "jam", "pam", "new", "aye", "ani", "and", "ibm", "yap", "can", "pyx", "tar", "kin", "fog", "hum", "pip", "cup", "dye", "lyx", "jog", "nun", "par", "wan", "fey", "bus", "oak", "bad", "ats", "set", "qom", "vat", "eat", "pus", "rev", "axe", "ion", "six", "ila", "lao", "mom", "mas", "pro", "few", "opt", "poe", "art", "ash", "oar", "cap", "lop", "may", "shy", "rid", "bat", "sum", "rim", "fee", "bmw", "sky", "maj", "hue", "thy", "ava", "rap", "den", "fla", "auk", "cox", "ibo", "hey", "saw", "vim", "sec", "ltd", "you", "its", "tat", "dew", "eva", "tog", "ram", "let", "see", "zit", "maw", "nix", "ate", "gig", "rep", "owe", "ind", "hog", "eve", "sam", "zoo", "any", "dow", "cod", "bed", "vet", "ham", "sis", "hex", "via", "fir", "nod", "mao", "aug", "mum", "hoe", "bah", "hal", "keg", "hew", "zed", "tow", "gog", "ass", "dem", "who", "bet", "gos", "son", "ear", "spy", "kit", "boy", "due", "sen", "oaf", "mix", "hep", "fur", "ada", "bin", "nil", "mia", "ewe", "hit", "fix", "sad", "rib", "eye", "hop", "haw", "wax", "mid", "tad", "ken", "wad", "rye", "pap", "bog", "gut", "ito", "woe", "our", "ado", "sin", "mad", "ray", "hon", "roy", "dip", "hen", "iva", "lug", "asp", "hui", "yak", "bay", "poi", "yep", "bun", "try", "lad", "elm", "nat", "wyo", "gym", "dug", "toe", "dee", "wig", "sly", "rip", "geo", "cog", "pas", "zen", "odd", "nan", "lay", "pod", "fit", "hem", "joy", "bum", "rio", "yon", "dec", "leg", "put", "sue", "dim", "pet", "yaw", "nub", "bit", "bur", "sid", "sun", "oil", "red", "doc", "moe", "caw", "eel", "dix", "cub", "end", "gem", "off", "yew", "hug", "pop", "tub", "sgt", "lid", "pun", "ton", "sol", "din", "yup", "jab", "pea", "bug", "gag", "mil", "jig", "hub", "low", "did", "tin", "get", "gte", "sox", "lei", "mig", "fig", "lon", "use", "ban", "flo", "nov", "jut", "bag", "mir", "sty", "lap", "two", "ins", "con", "ant", "net", "tux", "ode", "stu", "mug", "cad", "nap", "gun", "fop", "tot", "sow", "sal", "sic", "ted", "wot", "del", "imp", "cob", "way", "ann", "tan", "mci", "job", "wet", "ism", "err", "him", "all", "pad", "hah", "hie", "aim", "ike", "jed", "ego", "mac", "baa", "min", "com", "ill", "was", "cab", "ago", "ina", "big", "ilk", "gal", "tap", "duh", "ola", "ran", "lab", "top", "gob", "hot", "ora", "tia", "kip", "han", "met", "hut", "she", "sac", "fed", "goo", "tee", "ell", "not", "act", "gil", "rut", "ala", "ape", "rig", "cid", "god", "duo", "lin", "aid", "gel", "awl", "lag", "elf", "liz", "ref", "aha", "fib", "oho", "tho", "her", "nor", "ace", "adz", "fun", "ned", "coo", "win", "tao", "coy", "van", "man", "pit", "guy", "foe", "hid", "mai", "sup", "jay", "hob", "mow", "jot", "are", "pol", "arc", "lax", "aft", "alb", "len", "air", "pug", "pox", "vow", "got", "meg", "zoe", "amp", "ale", "bud", "gee", "pin", "dun", "pat", "ten", "mob"}))
}
