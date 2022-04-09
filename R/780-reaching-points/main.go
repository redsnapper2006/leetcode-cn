package main

import "fmt"

func reachingPoints(sx int, sy int, tx int, ty int) bool {
	for tx > sx && ty > sy && tx != ty {
		if tx > ty {
			tx %= ty
		} else {
			ty %= tx
		}
	}

	if tx == sx && ty == sy {
		return true
	}
	if tx == sx {
		return ty > sy && (ty-sy)%tx == 0
	}
	if ty == sy {
		return tx > sx && (tx-sx)%ty == 0
	}
	return false
}

func reachingPoints2(sx int, sy int, tx int, ty int) bool {
	if sx == 1 && tx == 1 && sy <= ty || sy == 1 && ty == 1 && sx <= tx {
		return true
	}
	for {
		// fmt.Println(sx, sy, tx, ty)
		if sx == tx && sy == ty {
			return true
		}
		if tx < sx || ty < sy {
			return false
		}

		if tx > ty {
			// tx %= ty
			ntx, nty := tx, ty
			for ntx > sx && ntx > nty {
				ntx -= nty
			}
			if ntx == tx && nty == ty {
				return false
			}
			tx, ty = ntx, nty
		} else if ty > tx {
			ntx, nty := tx, ty
			for nty > sy && nty > ntx {
				nty -= ntx
			}
			if ntx == tx && nty == ty {
				return false
			}
			tx, ty = ntx, nty
		} else {
			return false
		}
	}
}

func main() {
	fmt.Println(reachingPoints(1, 1, 3, 5))
	fmt.Println(reachingPoints(1, 5, 19, 5))
}
