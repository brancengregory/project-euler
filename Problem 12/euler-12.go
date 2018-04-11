package main

import (
	"fmt"
	"math"
)


func main() {
	
	ndivs := 0
	trinum := 0
	n := 0

	for i := 1; i < 1000000; i++ {
		trinum = (i*(i+1))/2
		n = int(math.Floor(math.Sqrt(float64(trinum))))
		vec := make([]int,n)
		for j,_ := range vec {
			if math.Mod(float64(trinum),float64(j))==0 {
				if trinum/(j) == (j) {
					ndivs += 1
				} else {
					ndivs += 2
				}
			} else {
				continue
			}
		}
		if ndivs >= 500 {
			break
		} else {
			ndivs = 0
			continue
		}
	}
	
	fmt.Println(trinum, ndivs)
	
}


