package main

import "fmt"

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

func makeDivSumArray(x int) []int {
	var arr []int
	for i := 1; i <= x; i++ {
		arr = append(arr, sumDivisors(i))
	}
	return arr
}

func isAmicable(x int) bool {
	var a int
	var b int
	a = sumDivisors(x)
	b = sumDivisors(a)
	if x == b && x != a {
		return true
	} else {
		return false
	}
}

func makeAmicableArray(x int) []int {
	var arr []int
	for i := 1; i <= x; i++ {
		if isAmicable(i) == true {
			arr = append(arr, i)
		}
	}
	return arr
}

func sumAmicableArray(x []int) int {
	var sum int
	for i := range x {
		sum += x[i]
	}
	return sum
}

func main() {
	amicableNums := makeAmicableArray(10000)
	amicableNumsSum := sumAmicableArray(amicableNums)

	test := isAmicable(6)
	test2 := makeAmicableArray(10000)

	fmt.Println(test, test2, amicableNumsSum)
}
