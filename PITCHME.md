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

---?code=go-sieve.go&lang=go
@snap[north-east]
## GO
@snapend

@[3]
@[20-32]
@[5-9]
@[11-18]

---?code=rust-sieve-thread.rs&lang=rust
@snap[north-east]
## Rust
@snapend
@[1-4]
@[28-45]
@[6-12]
@[14-26]

---
## Results

| 	|GO 	|Rust 	|
|-------|-------|-------|
|100 	|2.5 ms	|35 ms 	|
|1000 	|180 ms	|2.25 s	|

@snap[fragment]
<br>
Rust is about one magnitude **slower** than GO
@snapend
---
# Concurrency is not Parallelism
---
## Concurrency

![IMAGE](assets/img/CppConcurrencyInActionCh1Fig1-1TaskSwitching.jpg)

@ul[list-spaced-bullets]
- time-sharing
- multitasking
- task switching
@ulend
---
## Parallelism

![IMAGE](assets/img/CppConcurrencyInActionCh1Fig1-1Parallel.jpg)

---
## Concurrency and Parallelism

![IMAGE](assets/img/CppConcurrencyInActionCh1Fig1-2.jpg)
---
## Concurrency vs Parallelism

@ul[list-spaced-bullets text-08]
- **Concurrency** is the composition of independently executing processes
- **Parallelism** is the simultaneous execution of (possibly related) computations
@ulend

@ul[list-spaced-bullets text-08]
- **Concurrency** is about *dealing* with lots of things at once.
- **Parallelism** is about *doing* lots of things at once.
@ulend

