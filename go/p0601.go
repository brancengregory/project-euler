package main

import (
	"fmt"
	"math/big"
)

func main() {
	base := new(big.Int).SetInt64(6)
	exp := new(big.Int).SetInt64(10)
	test := new(big.Int).Exp(base, exp, nil)

	fmt.Println(test)
}

func isDivisible(x, n *big.Int) bool {
	ans := new(big.Int)
	if ans.Mod(x, n) == new(big.Int).SetInt64(0) {
		return (true)
	}
	return (false)
}

func streak(x *big.Int) *big.Int {
	done := false
	i := new(big.Int).SetInt64(1)
	for done == false {
		if isDivisible(x, i) == true {
			i.Add(i, new(big.Int).SetInt64(1))
		} else {
			i.Sub(i, new(big.Int).SetInt64(1))
			done = true
		}
	}
	return (i)
}

func pFunc() {

}
