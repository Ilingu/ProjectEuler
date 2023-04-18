package main

import "fmt"

func main() {
	x, y := 1, 2
	even_sum := 2

	for x+y < 4e6 {
		new_term := x + y
		if new_term%2 == 0 {
			even_sum += int(new_term)
		}

		x = y
		y = new_term
	}
	fmt.Println(even_sum)
}
