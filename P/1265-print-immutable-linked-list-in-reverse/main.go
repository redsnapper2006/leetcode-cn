package main

type ImmutableListNode struct {
}

func (this *ImmutableListNode) getNext() ImmutableListNode {
	// return the next node.
}

func (this *ImmutableListNode) printValue() {
	// print the value of this node.
}

func printLinkedListInReverse(head ImmutableListNode) {
	next := head.getNext()
	if next != nil {
		printLinkedListInReverse(next)
	}
	head.printValue()
}
