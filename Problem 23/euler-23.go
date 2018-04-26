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

func sumDivisors(x int) int {
	var divs []int
	divs = getDivisors(x)
	var sum int
	for i := range divs {
		sum += divs[i]
	}
	return sum
}

func isPerfectNumber(x int) bool {
	if sumDivisors(x) == x {
		return true
	}
	return false
}

func isAbundantNumber(x int) bool {
	sum := sumDivisors(x)
	if sum > x {
		return true
	}
	return false
}

func makeAbundantsArray(x int) []int {
	var arr []int
	for i := 1; i <= x; i++ {
		if isAbundantNumber(i) == true {
			arr = append(arr, i)
		}
	}
	return arr
}

func isSumOfAbundants(x int) bool {
	arr := makeAbundantsArray(x)
	if x < 24 {
		return false
	}
	for _, y := range arr {
		rem := x - y
		for _, z := range arr {
			if rem == z {
				return true
			}
		}
	}
	return false
}

func allSumsofAbundants(x int) [28123]bool {
	var tot [28123]bool
	arr := makeAbundantsArray(x)
	for _, i := range arr {
		for _, j := range arr {
			if i+j <= x {
				tot[i+j-1] = true
			}
		}
	}
	return tot
}

func main() {

	var sum int
	arr := allSumsofAbundants(28123)
	for i, val := range arr {
		if val == false {
			sum += i + 1
		}
	}

	/* test := isPerfectNumber(28)
	test2 := isAbundantNumber(12)
	test3 := makeAbundantsArray(200)
	test4 := isSumOfAbundants(12)
	test5 := allSumsofAbundants(400)

	fmt.Println(test, test2, test3[1], test4, test5, sum) */

	fmt.Println("The answer is:", sum)
}
