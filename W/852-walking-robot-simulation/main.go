package main

import "fmt"

func nextGreatestLetter(letters []byte, target byte) byte {
	s, e := 0, len(letters)-1
	m := s + (e-s)/2
	for s < e {
		m = s + (e-s)/2
		if letters[m] > target {
			if letters[s] < letters[m] {
				if letters[s] <= target {
					e = m - 1
				} else if letters[s] > target {
					s = m + 1
				}
			} else {
				e = m - 1
			}
		} else if letters[m] < target {
			if letters[s] <= letters[m] {
				s = m + 1
			} else {
				if letters[s] <= target {
					e = m - 1
				} else {
					s = m + 1
				}
			}
		} else {
			break
		}
	}
	fmt.Println(m)
	if letters[m] == target {
		if m < len(letters)-1 {
			return letters[m+1]
		} else {
			return letters[0]
		}
	} else if letters[m] < target {
		if m < len(letters)-1 {
			return letters[m+1]
		} else {
			return letters[0]
		}
	}

	if m > 0 {
		return letters[m-1]
	} else {
		return letters[len(letters)-1]
	}
}

func main() {
	fmt.Println(nextGreatestLetter([]byte{'c', 'f', 'j'}, 'a'))
	fmt.Println(nextGreatestLetter([]byte{'c', 'f', 'j'}, 'c'))
	fmt.Println(nextGreatestLetter([]byte{'c', 'f', 'j'}, 'd'))
	fmt.Println(nextGreatestLetter([]byte{'c', 'f', 'j'}, 'g'))
	fmt.Println(nextGreatestLetter([]byte{'c', 'f', 'j'}, 'j'))
	fmt.Println(nextGreatestLetter([]byte{'c', 'f', 'j'}, 'k'))
}
