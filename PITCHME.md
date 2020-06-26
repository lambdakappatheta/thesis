## Concurrent programming in Rust and Go

---

@snap[north span-100]
## Sieve of Eratosthenes
@snapend

@snap[south]
![IMAGE](assets/img/SieveOfErastothenes_start.jpg)
@snapend

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
