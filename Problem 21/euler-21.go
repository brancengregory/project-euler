package main

import (
	"fmt"
)

func getDivisors(x int) []int {
	var vec []int
	for i := 1; i < x; i++ {
		if x%i == 0 {
			vec = append(vec, i)
		}
	}
	return vec
}

func sumDivisors(x []int) int {
	var sum int
	for i, _ := range x {
		sum = sum + x[i]
	}
	return sum
}

func main() {

	test := getDivisors(220)
	test2 := sumDivisors(test)

	fmt.Println(test, test2)
}
