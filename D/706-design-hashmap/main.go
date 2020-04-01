package main

type MyHashMap struct {
	B [][]int
}

/** Initialize your data structure here. */
func Constructor() MyHashMap {
	b := make([][]int, 1000)
	for i := 0; i < 1000; i++ {
		b[i] = make([]int, 1000)
		for j := 0; j < 1000; j++ {
			b[i][j] = -1
		}
	}
	return MyHashMap{B: b}
}

/** value will always be non-negative. */
func (this *MyHashMap) Put(key int, value int) {
	idx := key / 1000
	offset := key % 1000
	this.B[idx][offset] = value
}

/** Returns the value to which the specified key is mapped, or -1 if this map contains no mapping for the key */
func (this *MyHashMap) Get(key int) int {
	idx := key / 1000
	offset := key % 1000
	return this.B[idx][offset]
}

/** Removes the mapping of the specified value key if this map contains a mapping for the key */
func (this *MyHashMap) Remove(key int) {
	idx := key / 1000
	offset := key % 1000
	this.B[idx][offset] = -1
}

func main() {
	fmt.Println("a")
}
