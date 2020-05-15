package main

import "fmt"

/*   Below is the interface for Iterator, which is already defined for you.
 *
 */
type Iterator struct {
}

func (this *Iterator) hasNext() bool {
	// Returns true if the iteration has more elements.
	return true
}

func (this *Iterator) next() int {
	// Returns the next element in the iteration.
	return 0
}

type PeekingIterator struct {
	Iter    *Iterator
	Preload []int
}

func Constructor(iter *Iterator) *PeekingIterator {
	return &PeekingIterator{Iter: iter, Preload: []int{}}
}

func (this *PeekingIterator) hasNext() bool {
	return len(this.Preload) > 0 || this.Iter.hasNext()
}

func (this *PeekingIterator) next() int {
	if len(this.Preload) > 0 {
		v := this.Preload[0]
		this.Preload = this.Preload[1:]
		return v
	}

	return this.Iter.next()
}

func (this *PeekingIterator) peek() int {
	if len(this.Preload) > 0 {
		return this.Preload[0]
	}
	v := this.Iter.next()
	this.Preload = append(this.Preload, v)
	return v
}

func main() {
	fmt.Println("a")
}
