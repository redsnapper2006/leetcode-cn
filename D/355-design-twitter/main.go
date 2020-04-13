package main

import (
	"fmt"
)

type Twitter struct {
	TQ []int
	UM map[int][]int
	FM map[int][]int
}

/** Initialize your data structure here. */
func Constructor() Twitter {
	return Twitter{TQ: []int{}, UM: make(map[int][]int), FM: make(map[int][]int)}
}

/** Compose a new tweet. */
func (this *Twitter) PostTweet(userId int, tweetId int) {
	this.TQ = append(this.TQ, tweetId)
	this.UM[userId] = append(this.UM[userId], len(this.TQ)-1)
}

/** Retrieve the 10 most recent tweet ids in the user's news feed. Each item in the news feed must be posted by users who the user followed or by the user herself. Tweets must be ordered from most recent to least recent. */
func (this *Twitter) GetNewsFeed(userId int) []int {
	fl := this.FM[userId]
	q := make([]int, len(fl)+1)
	copy(q, fl)
	q[len(q)-1] = userId
	idx := make([]int, len(q))
	for i := 0; i < len(idx); i++ {
		idx[i] = len(this.UM[q[i]]) - 1
	}

	var r []int
	for i := 0; i < 10; i++ {
		max := -1
		cIdx := -1
		for j := 0; j < len(idx); j++ {
			if idx[j] != -1 && max < this.UM[q[j]][idx[j]] {
				cIdx = j
				max = this.UM[q[j]][idx[j]]
			}
		}
		if max != -1 {
			r = append(r, this.TQ[max])
		}
		if cIdx != -1 {
			idx[cIdx]--
		}
	}
	return r
}

/** Follower follows a followee. If the operation is invalid, it should be a no-op. */
func (this *Twitter) Follow(followerId int, followeeId int) {
	if followerId == followeeId {
		return
	}
	fe := this.FM[followerId]
	for i := 0; i < len(fe); i++ {
		if fe[i] == followeeId {
			return
		}
	}
	this.FM[followerId] = append(this.FM[followerId], followeeId)
}

/** Follower unfollows a followee. If the operation is invalid, it should be a no-op. */
func (this *Twitter) Unfollow(followerId int, followeeId int) {
	fe := this.FM[followerId]
	idx := -1
	for i := 0; i < len(fe); i++ {
		if fe[i] == followeeId {
			idx = i
			break
		}
	}
	if idx != -1 {
		fe = append(fe[0:idx], fe[idx+1:]...)
		this.FM[followerId] = fe
	}
}

func main() {

	o := Constructor()
	o.PostTweet(1, 5)
	fmt.Println(o.GetNewsFeed(1))

	o.Follow(1, 2)
	o.PostTweet(2, 6)
	fmt.Println(o.GetNewsFeed(1))

	o.Unfollow(1, 2)
	fmt.Println(o.GetNewsFeed(1))
}
