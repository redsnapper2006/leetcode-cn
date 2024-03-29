package main

import "fmt"

/**
 * The read4 API is already defined for you.
 *
 *     read4 := func(buf4 []byte) int
 *
 * // Below is an example of how the read4 API can be called.
 * file := File("abcdefghijk") // File is "abcdefghijk", initially file pointer (fp) points to 'a'
 * buf4 := make([]byte, 4) // Create buffer with enough space to store characters
 * read4(buf4) // read4 returns 4. Now buf = ['a','b','c','d'], fp points to 'e'
 * read4(buf4) // read4 returns 4. Now buf = ['e','f','g','h'], fp points to 'i'
 * read4(buf4) // read4 returns 3. Now buf = ['i','j','k',...], fp points to end of file
 */
var solution = func(read4 func([]byte) int) func([]byte, int) int {
	// implement read below.
	return func(buf []byte, n int) int {
		buf4 := make([]byte, 4)
		cnt := read4(buf4)
		idx := 0
		for i := 0; i < 4 && i < cnt && idx < n; i++ {
			buf[idx] = buf4[i]
			idx++
		}

		for idx < n {
			buf4 := make([]byte, 4)
			cnt := read4(buf4)
			for i := 0; i < 4 && i < cnt && idx < n; i++ {
				buf[idx] = buf4[i]
				idx++
			}

			if cnt < 4 {
				break
			}
		}
		return idx
	}
}

func main() {
	fmt.Println()
}
