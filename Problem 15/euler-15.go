package main

import (
	"fmt"
	"strings"
	"strconv"
)


/*func factorial(n int) []int{
	var vec []int
	for i:=1; i<=n; i++ {
		vec = append(vec,i)
	}
	return vec
}
*/
func sumDigits(n int) []int64{
	//Convert int to string
	var s string = string(n)
	hold := strings.Split(s,"")
	//Convert strings to vector
	var result []int64
	for i:=0; i<=len(result); i++ {
		result = append(strconv.ParseInt(hold[i],10,64), result)
	}
	//Sum elements of vector
	return(result)
}


func main() {
	
	var test [24]int64
	
	
	fmt.Println(len(test))
}