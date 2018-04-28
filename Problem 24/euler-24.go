package main

import (
	"fmt"
	"math"
)

func Factorial(n int64) (ans int64) {
	if n > 0 {
		ans = n * Factorial(n-1)
		return ans
	}
	return 1
}

func Remove(slice []string, s int64) []string {
	return append(slice[:s], slice[s+1:]...)
}

func findNthPermutation(n int64, s []string) (ans []string) {
	length := int64(len(s))

	if n > Factorial(length) {
		fmt.Println("There are only", Factorial(int64(len(s))), "possible permutations... Use a smaller value for n.")
		return nil
	}

	fmt.Println("Length:", length)

	var cohort float64
	var rem = n
	var opts = s
	for i := int64(0); i <= int64(len(s)-1); i++ {
		cohort = math.Floor(float64(rem) / float64(Factorial(length-i)/length-i))
		fmt.Println("Cohort:", int64(cohort), "\n", "Ans:", ans, "\n")
		ans = append(ans, s[int64(cohort)])
		fmt.Println("Ans:", ans)
		opts = Remove(opts, int64(cohort))
		fmt.Println("Opts:", opts)
		rem -= int64(cohort) * (Factorial(length-i)/length - i)
		fmt.Println("Remainder:", rem)
	}
	fmt.Println("Remainder:", rem)
	return ans
}

func main() {
	nums := []string{"0", "1", "2", "3", "4", "5", "6", "7", "8", "9"}
	//nums := []string{"0", "1", "2"}

	fmt.Println(nums, Factorial(10), findNthPermutation(1000000, nums), len(nums))
}
