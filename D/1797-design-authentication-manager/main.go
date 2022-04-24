package main

type AuthenticationManager struct {
	M   map[string]int
	TTL int
}

func Constructor(timeToLive int) AuthenticationManager {
	return AuthenticationManager{M: map[string]int{}, TTL: timeToLive}
}

func (this *AuthenticationManager) Generate(tokenId string, currentTime int) {
	this.M[tokenId] = currentTime + this.TTL
}

func (this *AuthenticationManager) Renew(tokenId string, currentTime int) {
	v, ok := this.M[tokenId]
	if !ok {
		return
	}
	if v <= currentTime {
		delete(this.M, tokenId)
		return
	}
	this.M[tokenId] = currentTime + this.TTL
}

func (this *AuthenticationManager) CountUnexpiredTokens(currentTime int) int {
	for k, v := range this.M {
		if v <= currentTime {
			delete(this.M, k)
		}
	}
	return len(this.M)
}
