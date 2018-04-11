package main

import(
	"fmt"
)


func getDivisors(x int) []int {
	var vec []int
	for i:=1; i<x; i++{
		if x % i == 0 {
			vec = append(vec,i)
		}
	}
	return vec
}

func sumDivisors(x []int) int {
	var sum int
	for i,_ := range x {
		sum = sum + x[i]
	}
	return sum
}

func isAbundant(x int) bool {
	if x>=12{	
		a := getDivisors(x)
		b := sumDivisors(a)
		if b > x {
			return true
		}
	}
	return false
}


func main() {
	
	var abundvec []int
	for i:=1; i<28123; i++ {
		if isAbundant(i)==true {
			abundvec = append(abundvec,i)
		}
	}
	
	
	
	test := getDivisors(20)
	test2 := sumDivisors(test)
	test3 := isAbundant(20)
	
	fmt.Println(test,test2,test3,abundvec)
}