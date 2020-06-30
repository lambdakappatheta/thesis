package main

import (
	"fmt"
	"os"
	"strconv"
)

func Generate(ch chan<- int) {
	for i := 2; ; i++ {
		ch <- i
	}
}

func Filter(in <-chan int, out chan<- int, prime int) {
	for {
		i := <-in
		if i%prime != 0 {
			out <- i
		}
	}
}

func Sieve(n int) []int {
	var primes []int
	ch := make(chan int)
	go Generate(ch)

	for i := 0; i < n; i++ {
		prime := <-ch
		primes = append(primes, prime)

		ch1 := make(chan int)
		go Filter(ch, ch1, prime)
		ch = ch1
	}
	return primes
}

func main() {
	n, _ := strconv.Atoi(os.Args[1])
	primes := Sieve(n)
	fmt.Printf("%v\n", primes)
}
