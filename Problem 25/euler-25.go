package main

import (
	"fmt"
	"math/big"
)

func ithFibBig(x int) (ans *big.Int) {
	if x <= 0 {
		fmt.Println("Val must be of positive length... Insert a larger value.")
		return nil
	} else if x <= 2 {
		ans = big.NewInt(1)
	} else {
		ans = big.NewInt(0)
		ans.Add(ithFibBig(x-1), ithFibBig(x-2))
	}
	return ans
}

func main() {
	fmt.Println(ithFibBig(50))
}
