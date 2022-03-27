package main

import (
	"strconv"
	"strings"
)

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

type Codec struct {
}

func Constructor() Codec {
	return Codec{}
}

// Serializes a tree to a single string.
func (this *Codec) serialize(root *TreeNode) string {
	var rserialize func(root *TreeNode) string
	rserialize = func(root *TreeNode) string {
		if root == nil {
			return "nil,"
		}
		s := strconv.FormatInt(int64(root.Val), 10) + ","
		s += rserialize(root.Left)
		s += rserialize(root.Right)
		return s
	}
	return rserialize(root)
}

// Deserializes your encoded data to tree.
func (this *Codec) deserialize(data string) *TreeNode {
	var rdeserialize func(arr *[]string) *TreeNode
	rdeserialize = func(arr *[]string) *TreeNode {
		s := (*arr)[0]
		(*arr) = (*arr)[1:]
		if s == "nil" {
			return nil
		}
		v, _ := strconv.ParseInt(s, 10, 32)
		t := TreeNode{Val: int(v)}
		t.Left = rdeserialize(arr)
		t.Right = rdeserialize(arr)
		return &t
	}
	arr := strings.Split(data, ",")
	return rdeserialize(&arr)
}
