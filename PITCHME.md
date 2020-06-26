## Concurrent programming in Rust and Go

---
## Sieve of Eratosthenes
![IMAGE](assets/img/SieveOfErastothenes_start.jpg)

+++
## Sieve of Eratosthenes
![IMAGE](assets/img/SieveOfErastothenes_find-2.jpg)

+++
## Sieve of Eratosthenes
![IMAGE](assets/img/SieveOfErastothenes_mark-2.jpg)

+++
## Sieve of Eratosthenes
![IMAGE](assets/img/SieveOfErastothenes_find-3.jpg)

+++
## Sieve of Eratosthenes
![IMAGE](assets/img/SieveOfErastothenes_mark-3.jpg)

+++
## Sieve of Eratosthenes
![IMAGE](assets/img/SieveOfErastothenes_find-5.jpg)

+++
## Sieve of Eratosthenes
![IMAGE](assets/img/SieveOfErastothenes_mark-5.jpg)

+++
## Sieve of Eratosthenes
![IMAGE](assets/img/SieveOfErastothenes_find-7.jpg)

+++
## Sieve of Eratosthenes
![IMAGE](assets/img/SieveOfErastothenes_mark-7.jpg)

+++
## Sieve of Eratosthenes
![IMAGE](assets/img/SieveOfErastothenes_find-11.jpg)

---
@snap[span-100]
## Concurrent Sieve of Eratosthenes
@snapend

@snap[south-west]

@snap[fragment]
![IMAGE](assets/img/ConcurrentSieve_2-3.jpg)
@snapend

@snap[fragment]
![IMAGE](assets/img/ConcurrentSieve_2-3-5.jpg)
@snapend

@snapend

---
@snap[north-west]
## Concurrent Sieve
@snapend

@snap[west]

@img[fragment position=left](assets/img/ConcurrentSieve_2-3.jpg)
@img[fragment](assets/img/ConcurrentSieve_2-3-5.jpg)

@snapend

---
@snap[span-100]
## Concurrent Sieve of Eratosthenes
@snapend

@img[fragment position=left](assets/img/ConcurrentSieve_2-3.jpg)
@img[fragment](assets/img/ConcurrentSieve_2-3-5.jpg)

---
@snap[north span-100]
## Classic vs Concurrent
@snapend

@snap[west span-40 text-center]
### Classic

@ul[text-center]
- find all primes between 2 and x
- offline
@ulend
@snapend

@snap[east span-40]
concurrent sieve

find the first n primes
@snapend
---

Let's turn this into an online algorithm
ceheck primality on the fly

---?code=sieve.go
@[20-30]
@[5-10]
@[11-18]

---
```go zoom-18
package main

import "fmt"

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

func main() {
	ch := make(chan int)
	go Generate(ch)
	for i := 0; i < 10; i++ {
		prime := <-ch
		fmt.Println(prime)
		ch1 := make(chan int)
		go Filter(ch, ch1, prime)
		ch = ch1
	}
}

@[20-30]
@[5-10]
@[11-18]
```
