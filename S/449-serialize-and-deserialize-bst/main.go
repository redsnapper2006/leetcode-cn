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
	var postOrder func(node *TreeNode) string
	postOrder = func(node *TreeNode) string {
		if node == nil {
			return ""
		}

		v := ""
		if node.Left != nil {
			v += postOrder(node.Left) + ","
		}
		if node.Right != nil {
			v += postOrder(node.Right) + ","
		}
		return v + strconv.Itoa(node.Val)
	}

	return postOrder(root)
}

// Deserializes your encoded data to tree.
func (this *Codec) deserialize(data string) *TreeNode {
	arr := strings.Split(data, ",")
	buf := []int{}
	for _, v := range arr {
		if v == "" {
			continue
		}
		i, _ := strconv.Atoi(v)
		buf = append(buf, i)
	}

	var buildTree func(buf []int) *TreeNode
	buildTree = func(buf []int) *TreeNode {
		if len(buf) == 0 {
			return nil
		}
		key := buf[len(buf)-1]
		idx := 0
		for idx < len(buf)-1 && buf[idx] < key {
			idx++
		}
		left := buildTree(buf[:idx])
		var right *TreeNode = nil
		if idx < len(buf)-1 {
			right = buildTree(buf[idx : len(buf)-1])
		}
		return &TreeNode{
			Val:   key,
			Left:  left,
			Right: right,
		}
	}
	return buildTree(buf)
}
