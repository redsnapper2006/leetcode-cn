package main

import (
	"crypto/md5"
	"encoding/base64"
	"fmt"
	"strings"
)

const (
	PREFIX = "https://leetcode.com/"
)

type Codec struct {
	MD5SUM map[string]string
}

func Constructor() Codec {
	return Codec{MD5SUM: make(map[string]string)}
}

// Encodes a URL to a shortened URL.
func (this *Codec) encode(longUrl string) string {
	short := md5.Sum([]byte(longUrl))
	b := make([]byte, len(short))
	copy(b, short[0:])
	key := base64.StdEncoding.EncodeToString(b)
	this.MD5SUM[key] = longUrl
	return PREFIX + key
}

// Decodes a shortened URL to its original URL.
func (this *Codec) decode(shortUrl string) string {
	short := strings.Replace(shortUrl, PREFIX, "", -1)
	// short := shortUrl[idx+1:]
	return this.MD5SUM[short]
}

func main() {
	o := Constructor()
	b := o.encode("http://www.example.com/book.aspx")
	fmt.Println(b)
	fmt.Println(o.decode(b))
}
