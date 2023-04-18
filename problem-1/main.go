package main

import "fmt"

func main() {
	result := mutipleSum(3, 1000) + mutipleSum(5, 1000) - mutipleSum(15, 1000)
	fmt.Println(result)
}

func mutipleSum(mutiple uint, lim uint) (sum uint) {
	for i := uint(0); mutiple*(i+1) < lim; i++ {
		sum += mutiple * (i + 1)
	}
	return sum
}
