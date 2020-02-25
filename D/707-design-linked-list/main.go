package main

type ListNode struct {
	Val  int
	Next *ListNode
}

type MyLinkedList struct {
	Head *ListNode
}

func Constructor() MyLinkedList {
	return MyLinkedList{Head: nil}
}

/** Get the value of the index-th node in the linked list. If the index is invalid, return -1. */
func (this *MyLinkedList) Get(index int) int {
	if this.Head == nil {
		return -1
	}
	p := this.Head
	c := 0
	for p != nil && c < index {
		p = p.Next
		c++
	}
	if p == nil {
		return -1
	} else {
		return p.Val
	}
}

/** Add a node of value val before the first element of the linked list. After the insertion, the new node will be the first node of the linked list. */
func (this *MyLinkedList) AddAtHead(val int) {
	if this.Head == nil {
		this.Head = &ListNode{Val: val, Next: nil}
	} else {
		p := ListNode{Val: val, Next: this.Head}
		this.Head = &p
	}
}

/** Append a node of value val to the last element of the linked list. */
func (this *MyLinkedList) AddAtTail(val int) {
	if this.Head == nil {
		this.Head = &ListNode{Val: val, Next: nil}
	} else {
		p := this.Head
		for p.Next != nil {
			p = p.Next
		}
		q := ListNode{Val: val, Next: nil}
		p.Next = &q
	}
}

/** Add a node of value val before the index-th node in the linked list. If index equals to the length of linked list, the node will be appended to the end of linked list. If index is greater than the length, the node will not be inserted. */
func (this *MyLinkedList) AddAtIndex(index int, val int) {
	if index == 0 {
		this.AddAtHead(val)
	} else {
		if this.Head != nil {

			p := this.Head
			c := 0
			for p.Next != nil && c < index-1 {
				p = p.Next
				c++
			}
			if c == index-1 {
				if p.Next == nil {
					q := ListNode{Val: val, Next: nil}
					p.Next = &q
				} else {
					q := ListNode{Val: val, Next: p.Next}
					p.Next = &q
				}
			}
		}
	}
}

/** Delete the index-th node in the linked list, if the index is valid. */
func (this *MyLinkedList) DeleteAtIndex(index int) {
	if this.Head != nil {

		if index == 0 {
			this.Head = this.Head.Next
			return
		}

		p := this.Head
		c := 0

		for p.Next != nil && c < index-1 {
			p = p.Next
			c++
		}

		if c == index-1 && p.Next != nil {
			p.Next = p.Next.Next
		}
	}
}

func main() {

	o := Constructor()
	o.AddAtHead(1)
	o.AddAtTail(3)
	o.AddAtIndex(1, 2)
	o.Get(1)
	o.DeleteAtIndex(1)
	o.Get(1)

}
