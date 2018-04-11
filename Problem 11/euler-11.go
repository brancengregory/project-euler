package main

//Dependencies:

import (
	"fmt"
	//"reflect"
	"strings"
	"strconv"
	//"unicode/utf8"
	//"math/big"
)

//Min and Max Functions:

func min(x ...int) int {
	min := x[0]
	for _, i := range x[1:] {
		if i < min {
			min = i
		}
	}
	return min
}

func max(x ...int) int {
	max := x[0]
	for _, i := range x[1:] {
		if i > max {
			max = i
		}
	}
	return max
}

//Main function:

func main() {
	
	var rawData string = "08 02 22 97 38 15 00 40 00 75 04 05 07 78 52 12 50 77 91 0849 49 99 40 17 81 18 57 60 87 17 40 98 43 69 48 04 56 62 0081 49 31 73 55 79 14 29 93 71 40 67 53 88 30 03 49 13 36 6552 70 95 23 04 60 11 42 69 24 68 56 01 32 56 71 37 02 36 9122 31 16 71 51 67 63 89 41 92 36 54 22 40 40 28 66 33 13 8024 47 32 60 99 03 45 02 44 75 33 53 78 36 84 20 35 17 12 5032 98 81 28 64 23 67 10 26 38 40 67 59 54 70 66 18 38 64 7067 26 20 68 02 62 12 20 95 63 94 39 63 08 40 91 66 49 94 2124 55 58 05 66 73 99 26 97 17 78 78 96 83 14 88 34 89 63 7221 36 23 09 75 00 76 44 20 45 35 14 00 61 33 97 34 31 33 9578 17 53 28 22 75 31 67 15 94 03 80 04 62 16 14 09 53 56 9216 39 05 42 96 35 31 47 55 58 88 24 00 17 54 24 36 29 85 5786 56 00 48 35 71 89 07 05 44 44 37 44 60 21 58 51 54 17 5819 80 81 68 05 94 47 69 28 73 92 13 86 52 17 77 04 89 55 4004 52 08 83 97 35 99 16 07 97 57 32 16 26 26 79 33 27 98 6688 36 68 87 57 62 20 72 03 46 33 67 46 55 12 32 63 93 53 6904 42 16 73 38 25 39 11 24 94 72 18 08 46 29 32 40 62 76 3620 69 36 41 72 30 23 88 34 62 99 69 82 67 59 85 74 04 36 1620 73 35 29 78 31 90 01 74 31 49 71 48 86 81 16 23 57 05 5401 70 54 71 83 51 54 69 16 92 33 48 61 43 52 01 89 19 67 48"
	
	rawData = strings.Replace(rawData," ","",-1)
	
	var mat [20][20]int
	var k int = 0
	for i:=0; i<20; i++ {
		for j:=0; j<20; j++ {
			temp1 := rawData[k:k+2]
			temp2, _ := strconv.ParseInt(temp1,10,64)
			mat[i][j] = int(temp2)
			k+=2
		}
	}
	
	l := 4
	m := len(mat)-1
	n := len(mat)
	df := make([]int, 8)
	var maxprod int = 0
	prod1 := 0
	prod2 := 0
	prod3 := 0
	prod4 := 0
	prod5 := 0
	prod6 := 0
	prod7 := 0
	prod8 := 0
	
	
	for i:=0; i<n; i++ {
		for j:=0; j<n; j++ {
			if i<=(l-2) {
				if j<=(l-2) {
					df = []int{1,2,3}
					for _, dfd := range df {
						if dfd==1 {
							prod1 = mat[i][j] * mat[i][j+1] * mat[i][j+2] * mat[i][j+3]
						} else if dfd==2 {
							prod2 = mat[i][j] * mat[i+1][j+1] * mat[i+2][j+2] * mat[i+3][j+3]
						} else if dfd==3 {
							prod3 = mat[i][j] * mat[i+1][j] * mat[i+2][j] * mat[i+3][j]
						}
					}
					if max(prod1,prod2,prod3) > maxprod {
						maxprod = max(prod1,prod2,prod3)
					}
				} else if j>=(m-l+2) {
					df = []int{3,4,5}
					for _, dfd := range df {
						if dfd==3 {
							prod1 = mat[i][j] * mat[i+1][j] * mat[i+2][j] * mat[i+3][j]
						} else if dfd==4 {
							prod2 = mat[i][j] * mat[i+1][j-1] * mat[i+2][j-2] * mat[i+3][j-3]
						} else if dfd==5 {
							prod3 = mat[i][j] * mat[i][j-1] * mat[i][j-2] * mat[i][j-3]
						}
					}
					if max(prod1,prod2,prod3) > maxprod {
						maxprod = max(prod1,prod2,prod3)
					}
				} else {
					df = []int{1,2,3,4,5}
					for _, dfd := range df {
						if dfd==1 {
							prod1 = mat[i][j] * mat[i][j+1] * mat[i][j+2] * mat[i][j+3]
						} else if dfd==2 {
							prod2 = mat[i][j] * mat[i+1][j+1] * mat[i+2][j+2] * mat[i+3][j+3]
						} else if dfd==3 {
							prod3 = mat[i][j] * mat[i+1][j] * mat[i+2][j] * mat[i+3][j]
						} else if dfd==4 {
							prod4 = mat[i][j] * mat[i+1][j-1] * mat[i+2][j-2] * mat[i+3][j-3]
						} else if dfd==5 {
							prod5 = mat[i][j] * mat[i][j-1] * mat[i][j-2] * mat[i][j-3]
						}
					}
					if max(prod1,prod2,prod3,prod4,prod5) > maxprod {
						maxprod = max(prod1,prod2,prod3,prod4,prod5)
					}
				}
			} else if i>=(m-l+2) {
				if j<=(l-2) {
					df = []int{1,7,8}
					for _, dfd := range df {
						if dfd==1 {
							prod1 = mat[i][j] * mat[i][j+1] * mat[i][j+2] * mat[i][j+3]
						} else if dfd==7 {
							prod2 = mat[i][j] * mat[i-1][j] * mat[i-2][j] * mat[i-3][j]
						} else if dfd==8 {
							prod3 = mat[i][j] * mat[i-1][j+1] * mat[i-2][j+2] * mat[i-3][j+3]
						}
					}
					if max(prod1,prod2,prod3) > maxprod {
						maxprod = max(prod1,prod2,prod3)
					}
				} else if j>=(m-l+2) {
					df = []int{5,6,7}
					for _, dfd := range df {
						if dfd==5 {
							prod1 = mat[i][j] * mat[i][j-1] * mat[i][j-2] * mat[i][j-3]
						} else if dfd==6 {
							prod2 = mat[i][j] * mat[i-1][j-1] * mat[i-2][j-2] * mat[i-3][j-3]
						} else if dfd==7 {
							prod3 = mat[i][j] * mat[i-1][j] * mat[i-2][j] * mat[i-3][j]
						}
					}
					if max(prod1,prod2,prod3) > maxprod {
						maxprod = max(prod1,prod2,prod3)
					}
				} else {
					df = []int{1,5,6,7,8}
					for _, dfd := range df {
						if dfd==1 {
							prod1 = mat[i][j] * mat[i][j+1] * mat[i][j+2] * mat[i][j+3]
						} else if dfd==5 {
							prod2 = mat[i][j] * mat[i][j-1] * mat[i][j-2] * mat[i][j-3]
						} else if dfd==6 {
							prod3 = mat[i][j] * mat[i-1][j-1] * mat[i-2][j-2] * mat[i-3][j-3]
						} else if dfd==7 {
							prod4 = mat[i][j] * mat[i-1][j] * mat[i-2][j] * mat[i-3][j]
						} else if dfd==8 {
							prod5 = mat[i][j] * mat[i-1][j+1] * mat[i-2][j+2] * mat[i-3][j+3]
						}
					}
					if max(prod1,prod2,prod3,prod4,prod5) > maxprod {
						maxprod = max(prod1,prod2,prod3,prod4,prod5)
					}
				}
			} else {
				if j<=(l-2) {
					df = []int{1,2,3,7,8}
					for _, dfd := range df {
						if dfd==1 {
							prod1 = mat[i][j] * mat[i][j+1] * mat[i][j+2] * mat[i][j+3]
						} else if dfd==2 {
							prod2 = mat[i][j] * mat[i+1][j+1] * mat[i+2][j+2] * mat[i+3][j+3]
						} else if dfd==3 {
							prod3 = mat[i][j] * mat[i+1][j] * mat[i+2][j] * mat[i+3][j]
						} else if dfd==7 {
							prod4 = mat[i][j] * mat[i-1][j] * mat[i-2][j] * mat[i-3][j]
						} else if dfd==8 {
							prod5 = mat[i][j] * mat[i-1][j+1] * mat[i-2][j+2] * mat[i-3][j+3]
						}
					}
					if max(prod1,prod2,prod3,prod4,prod5) > maxprod {
						maxprod = max(prod1,prod2,prod3,prod4,prod5)
					}
				} else if j>=(m-l+2) {
					df = []int{3,4,5,6,7}
					for _, dfd := range df {
						if dfd==3 {
							prod1 = mat[i][j] * mat[i+1][j] * mat[i+2][j] * mat[i+3][j]
						} else if dfd==4 {
							prod2 = mat[i][j] * mat[i+1][j-1] * mat[i+2][j-2] * mat[i+3][j-3]
						} else if dfd==5 {
							prod3 = mat[i][j] * mat[i][j-1] * mat[i][j-2] * mat[i][j-3]
						} else if dfd==6 {
							prod4 = mat[i][j] * mat[i-1][j-1] * mat[i-2][j-2] * mat[i-3][j-3]
						} else if dfd==7 {
							prod5 = mat[i][j] * mat[i-1][j] * mat[i-2][j] * mat[i-3][j]
						}
					}
					if max(prod1,prod2,prod3,prod4,prod5) > maxprod {
						maxprod = max(prod1,prod2,prod3,prod4,prod5)
					}
				} else {
					df = []int{1,2,3,4,5,6,7,8}
					for _, dfd := range df {
						if dfd==1 {
							prod1 = mat[i][j] * mat[i][j+1] * mat[i][j+2] * mat[i][j+3]
						} else if dfd==2 {
							prod2 = mat[i][j] * mat[i+1][j+1] * mat[i+2][j+2] * mat[i+3][j+3]
						} else if dfd==3 {
							prod3 = mat[i][j] * mat[i+1][j] * mat[i+2][j] * mat[i+3][j]
						} else if dfd==4 {
							prod4 = mat[i][j] * mat[i+1][j-1] * mat[i+2][j-2] * mat[i+3][j-3]
						} else if dfd==5 {
							prod5 = mat[i][j] * mat[i][j-1] * mat[i][j-2] * mat[i][j-3]
						} else if dfd==6 {
							prod6 = mat[i][j] * mat[i-1][j-1] * mat[i-2][j-2] * mat[i-3][j-3]
						} else if dfd==7 {
							prod7 = mat[i][j] * mat[i-1][j] * mat[i-2][j] * mat[i-3][j]
						} else if dfd==8 {
							prod8 = mat[i][j] * mat[i-1][j+1] * mat[i-2][j+2] * mat[i-3][j+3]
						}
					}
					if max(prod1,prod2,prod3,prod4,prod5,prod6,prod7,prod8) > maxprod {
						maxprod = max(prod1,prod2,prod3,prod4,prod5,prod6,prod7,prod8)
					}
				}
			}
		}
	}

	fmt.Println(maxprod)
	
}