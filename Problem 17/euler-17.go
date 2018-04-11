package main

import (
	"fmt"
	"strconv"
)

func constructNumber(x int) string {
	
	ones := []string{"zero","one","two","three","four","five","six","seven","eight","nine"}
	teens := []string{"eleven","twelve","thirteen","fourteen","fifteen",
					  "sixteen","seventeen","eighteen","nineteen"}
	tens := []string{"ten","twenty","thirty","forty","fifty","sixty","seventy","eighty","ninety"}
	hundred := "hundred"
	and := "and"
	thousand := "thousand"
	
	var num string
	var a int
	var b int
	var c int
	str := strconv.Itoa(x)
	
	if(len(str)==1) {
		num = ones[x]
	}
	if(len(str)==2){
		a,_ = strconv.Atoi(str[0:1])
		b,_ = strconv.Atoi(str[1:])
		if(b==0){
			num = tens[a-1]
		} else{
			if(a==1){
				num = teens[b-1]
			} else{
				num = tens[a-1]+ones[b]
			}
		}
	}
	if(len(str)==3){
		a,_ = strconv.Atoi(str[0:1])
		b,_ = strconv.Atoi(str[1:2])
		c,_ = strconv.Atoi(str[2:])
		num = ones[a] + hundred
		if(b==0 && c==0){
			return(num)
		}
		if(b==0 && c!=0){
			num = num + and + ones[c]
		}
		if(b!=0 && c==0){
			num = num + and + tens[b-1]
		}
		if(b!=0 && c!=0){
			if(b==1){
				num = num + and + teens[c-1]
			} else{
				num = num + and + tens[b-1] + ones[c]
			}
			
		}
	}
	if(len(str)==4){
		num = ones[1] + thousand
	}
	
	return num
}


func main() {
	
	answer := ""
	for i:=1; i<=1000; i++ {
		answer = answer + constructNumber(i)
	}
	
	fmt.Println(len(answer))
}