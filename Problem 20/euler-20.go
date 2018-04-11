package main

import(
	"fmt"
	"math/big"
	"strconv"
)

/*func factorial(n int) []int{
	var vec []int
	for i:=1; i<=n; i++ {
		vec = append(vec,i)
	}
	return vec
}*/

func Factorial(n int64)(result *big.Int) {
	if (n > 0) {
		var f big.Int
		result = f.MulRange(1,n)
		return(result)
	}
	return big.NewInt(1)
}

func main(){
	
	str := Factorial(100).String()
	var ans int64
	for i,_ := range str {
		num, _ := strconv.ParseInt(str[i:i+1],10,0)
		ans = ans + num
		//fmt.Println(ans)
	}
	
	
	fmt.Println(ans)
}