package main

import (
	"fmt"
)


func collatzit(n int) int{
	if(n%2==0){
		n = (n/2)
	} else{
		n = (3*n)+1
	}
	return(n)
}

func collatzseq(n int) []int{
	vec := make([]int, 1)
	vec[0] = n
	for {
		if n==1 {
			return(vec)
		} else {
			n = collatzit(n)
			vec = append(vec, n)
		}
	}
	return(vec)
}

func longestseq(thresh int) int{
	num := 0
	length := 0
	for i:=2; i<thresh; i++ {
		if len(collatzseq(i))>length {
			length = len(collatzseq(i))
			num = i
		}
	}
	return(num)
}


func main() {
	
	result := longestseq(1000000)
	
	fmt.Println("The answer is:",result)
}