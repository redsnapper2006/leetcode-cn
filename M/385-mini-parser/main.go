package main

type NestedInteger struct {
}

// Return true if this NestedInteger holds a single integer, rather than a nested list.
func (n NestedInteger) IsInteger() bool {
	return false
}

// Return the single integer that this NestedInteger holds, if it holds a single integer
// The result is undefined if this NestedInteger holds a nested list
// So before calling this method, you should have a check
func (n NestedInteger) GetInteger() int {
	return 0
}

// Set this NestedInteger to hold a single integer.
func (n *NestedInteger) SetInteger(value int) {}

// Set this NestedInteger to hold a nested list and adds a nested integer to it.
func (n *NestedInteger) Add(elem NestedInteger) {}

// Return the nested list that this NestedInteger holds, if it holds a nested list
// The list length is zero if this NestedInteger holds a single integer
// You can access NestedInteger's List element directly if you want to modify it
func (n NestedInteger) GetList() []*NestedInteger {
	return nil
}

func deserialize(s string) *NestedInteger {
	var root *NestedInteger
	stack := []byte{}
	pstack := []*NestedInteger{}
	for i := 0; i < len(s); i++ {
		if s[i] >= byte('0') && s[i] <= byte('9') || s[i] == byte('-') || s[i] == byte('[') {
			stack = append(stack, s[i])
			if s[i] == byte('[') {
				var cur *NestedInteger
				if len(pstack) > 0 {
					pstack[len(pstack)-1].Add(NestedInteger{})
					cur = pstack[len(pstack)-1].GetList()[len(pstack[len(pstack)-1].GetList())-1]
				} else {
					cur = new(NestedInteger)
					root = cur
				}
				pstack = append(pstack, cur)
			}
		} else if s[i] == byte(',') && s[i-1] >= byte('0') && s[i-1] <= byte('9') {
			num := 0
			times := 1
			for len(stack) > 0 && stack[len(stack)-1] != byte('[') && stack[len(stack)-1] != byte('-') {
				num += times * int(stack[len(stack)-1]-byte('0'))
				times *= 10
				stack = stack[0 : len(stack)-1]
			}
			if len(stack) > 0 && stack[len(stack)-1] == byte('-') {
				num = -num
				stack = stack[0 : len(stack)-1]
			}
			cur := new(NestedInteger)
			cur.SetInteger(num)
			pstack[len(pstack)-1].Add(*cur)
		} else if s[i] == byte(']') {
			if stack[len(stack)-1] >= byte('0') && stack[len(stack)-1] <= byte('9') {
				num := 0
				times := 1
				for len(stack) > 0 && stack[len(stack)-1] != byte('[') && stack[len(stack)-1] != byte('-') {
					num += times * int(stack[len(stack)-1]-byte('0'))
					times *= 10
					stack = stack[0 : len(stack)-1]
				}
				if len(stack) > 0 && stack[len(stack)-1] == byte('-') {
					num = -num
					stack = stack[0 : len(stack)-1]
				}
				cur := new(NestedInteger)
				cur.SetInteger(num)
				pstack[len(pstack)-1].Add(*cur)
			}
			stack = stack[0 : len(stack)-1]
			pstack = pstack[0 : len(pstack)-1]
		}
	}

	if len(stack) > 0 {
		num := 0
		times := 1
		for len(stack) > 0 && stack[len(stack)-1] != byte('-') {
			num += times * int(stack[len(stack)-1]-byte('0'))
			times *= 10
			stack = stack[0 : len(stack)-1]
		}
		if len(stack) > 0 && stack[len(stack)-1] == byte('-') {
			num = -num
			stack = stack[0 : len(stack)-1]
		}
		cur := new(NestedInteger)
		cur.SetInteger(num)
		return cur
	}

	return root
}
