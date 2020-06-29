## Concurrent programming in Rust and Go

---
## Sieve of Eratosthenes

---
@snap[north span-100]
## Sieve of Eratosthenes
@snapend
The classic version
<br>
@ul[list-no-bullets text-08]
- Goal: Find all the prime numbers between 2 and *x*
- <br>
- Algorithm:
- Create an array containing all positive integers between 2 and *x*.
- Consider each number in the array from smallest to largest.
- **If the next number *num* is unmarked, then *num* is a prime**! Mark all the multiples of *num* in the array!
- Repeat until all elements of the array have been considered.

@ulend

---
@snap[north span-100]
## Sieve of Eratosthenes
@snapend
<br>
<br>
![classic-empty](assets/img/classic/classic-empty.png)
![classic-2-find](assets/img/classic/classic-2-find.png)
![classic-2-mark](assets/img/classic/classic-2-mark.png)
![classic-3-find](assets/img/classic/classic-3-find.png)
![classic-3-mark](assets/img/classic/classic-3-mark.png)
![classic-5-find](assets/img/classic/classic-5-find.png)

---
@snap[north span-100]
## Sieve of Eratosthenes
@snapend
<br>
@ul[list-no-bullets text-08]
- Goal: @css[fragment](Find the first *n* prime numbers)
- <br>
- Algorithm:
- Keep the primes found so far in an array *primes*.
- Consider all positive integers one by one starting from 2.
- **If a number *num* is not divisible by any of the primes found so far, then *num* is a prime**!
- Add *num* to *primes* and continue until *n* primes have been found.
@ulend

---
## Sieve of Eratosthenes
![seq3](assets/img/seq/seq3.png)

+++
## Sieve of Eratosthenes
![seq3-after](assets/img/seq/seq3-after.png)

+++
## Sieve of Eratosthenes
![seq4](assets/img/seq/seq4.png)

+++
## Sieve of Eratosthenes
![seq5](assets/img/seq/seq5.png)

+++
## Sieve of Eratosthenes
![seq5-after](assets/img/seq/seq5-after.png)

+++
## Sieve of Eratosthenes
![seq6](assets/img/seq/seq6.png)

+++
## Sieve of Eratosthenes
![seq7](assets/img/seq/seq7.png)

+++
## Sieve of Eratosthenes
![seq7-after](assets/img/seq/seq7-after.png)

+++
## Sieve of Eratosthenes
![seq8](assets/img/seq/seq8.png)

+++
## Sieve of Eratosthenes
![seq9](assets/img/seq/seq9.png)

+++
## Sieve of Eratosthenes
![seq25](assets/img/seq/seq25.png)

---?code=rust/sequential/src/lib.rs&lang=rust
@snap[north-east]
## Rust
@snapend

@[1]
@[2,4,7]
@[2]
@[4-5]
@[23-24]
@[7-8,10,13-16,19-21]
@[25]

---
## Concurrent Sieve

@img[fragment](assets/img/concurrent/concurrent_2-3.png)
@img[fragment](assets/img/concurrent/concurrent_2-3-5.png)
@img[fragment](assets/img/concurrent/concurrent_2-3-5-7.png)

---
## Concurrent Sieve

@img[fragment](assets/img/concurrent/concurrent_2-3-5-7-before.png)
@img[fragment](assets/img/concurrent/concurrent_2-3-5-7-after.png)

---?code=go-sieve.go&lang=go
@snap[north-east]
## GO
@snapend

@[3]
@[5-9]
@[11-18]
@[20-32]

---?code=rust-sieve-thread.rs&lang=rust
@snap[north-east]
## Rust
@snapend
@[1-4]
@[6-12]
@[14-26]
@[28-45]

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
## Concurrency is about structure.
## Parallelism is about execution.

---
## Concurrency

@ul[list-no-bullets text-08]
- is about **structure**
@ulend













---
---
## Parallelism

![IMAGE](assets/img/CppConcurrencyInActionCh1Fig1-1Parallel.jpg)

@ul[list-spaced-bullets]
- only possible with hardware support (multiple cores, processors)
- increase throughput
@ulend

---
## Concurrency

![IMAGE](assets/img/CppConcurrencyInActionCh1Fig1-1TaskSwitching.jpg)

similar 
@ul[]
- time-sharing
- multitasking
- task switching
@ulend
---
@snap[north]
## Multitasking
Bake a cake and clean your room!
@snapend

@snap[south-west span-50 text-center]
@css[fragment](bake the cake first)
@css[fragment](clean your room afterwards)
@snapend

@snap[south-east span-50 text-center]
@css[fragment](prepare the ingredients and put the cake in the oven)
@css[fragment](clean your room in the meantime)
@css[fragment](finish the cake)
@snapend



---
@snap[north-west font-raleway-heavy text-blue text-22]
Multitasking
@snapend

@snap[west span-50 text-center]
bake the cake first
clean your room afterwards
@snapend

@snap[east span-50 text-center]
prepare the ingredients and put the cake in the oven
clean your room in the meantime
finish the cake
@snapend
---
@snap[north font-raleway-heavy text-blue text-22]
Multitasking
Problem: You need to bake a cake and clean your room!
@snapend

@ul[south-west span-50 text-center list-no-bullets]
- bake the cake first
- clean your room afterwards
@ulend

@ul[south-east span-50 text-center list-no-bullets]
- prepare the ingredients and put the cake in the oven
- clean your room in the meantime
- finish the cake
@ulend
---
@snap[north]
## Multitasking
Problem: You need to bake a cake and clean your room!
@snapend

@snap[south-west list-spaced-bullets]
@ul[text-08]
- bake the cake first
- clean your room afterwards
@ulend
@snapend

@snap[south-east list-spaced-bullets]
@ul[text-08]
- prepare the ingredients and put the cake in the oven
- clean your room in the meantime
- finish the cake
@ulend
@snapend
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

---
## problem

![IMAGE](assets/img/waza-slide12.jpg)
---
## fine-grained concurrency

![IMAGE](assets/img/waza-slide19.jpg)
---
## more parallelization

![IMAGE](assets/img/waza-slide22.jpg)

---
## intermaediate summary

@ul[list-spaced-bullets]
- *real parallelism* is only possible with hardware support (with multiple cores)
- the main benefit from *real parallelism* is increased throughput
- with *concurrencly* parallelism can be simulated
- 
@ulend

