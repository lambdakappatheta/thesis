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

---?code=rust/thread-sieve/src/lib.rs&lang=rust
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
## Concurrency is about structure
## Parallelism is about execution

---
![IMAGE](assets/img/ConcurrencyIsNotParallelism/s12.jpg)

burn the books

---
![IMAGE](assets/img/ConcurrencyIsNotParallelism/s13.jpg)

a friend drops by

---
![IMAGE](assets/img/ConcurrencyIsNotParallelism/s18.jpg)

load the cart
carry the cart
load the incinerator

---
![IMAGE](assets/img/ConcurrencyIsNotParallelism/s19.jpg)

load the cart
carry the loaded cart
return the empty cart
load the incinerator

---
The more people are around, the faster the process goes.

Single core performance has reached its limits.
But the number of cores in a single processor has been increasing rapidly.

Good (concurrent) design allows you to:
Sit back, buy new hardware and watch your program run faster!
(like in the Good Ol' Days)

---
![IMAGE](assets/img/ConcurrencyIsNotParallelism/s13.jpg)

---
![IMAGE](assets/img/ConcurrencyIsNotParallelism/s14.jpg)

---
![IMAGE](assets/img/ConcurrencyIsNotParallelism/s15.jpg)

---
![IMAGE](assets/img/MapReduce.png)

---
![IMAGE](assets/img/ConcurrencyIsNotParallelism/s22.jpg)

---
Let's get realistic

So far we were concerned with having too many workers but too few tasks to keep all of them busy. Therefore we needed to split up complex tasks into smaller (partially) independent pieces.

But for many years, computer scientists have had the problem of having too few workers and a lot of tasks to run. Actually even nowadays, the best affordable chips still do not have more than a few dozens of cores.

---
Ideally, the concurrent sieve algorithm implemented with threads will get blazingly fast in the future because there will be enough hardware parallelism available to assign each task to a dedicated computational unit. 

But for the time being, we would still like to be able to write a relatively efficient version of the concurrent sieve algorithm. Looking at GO's performance, it is obvious that there must be a way to improve Rust's performance!

---
Concurrency revisited

We said concurrency is about structure and that a well designed concurrent system can make heavy use of available hardware parallelism in order to improve performance dramatically.

But, concurrent structure can also be extremely beneficiary on single core CPU's.

How is that?

---
Concurrency revisited

Write an essay, clean your place and bake a cake.

Possible structures:
1)
Task 1: write an essay
Task 2: clean your place
Task 3: bake a cake

2)
Task 1.1: brainstorm
Task 1.2: write the introduction
Task 1.3: write the main part
Task 1.4: write the conclusion
Task 2.1: clean downstairs
Task 2.2: clean upstairs
Task 2.3: clean the basement
Task 2.3: clean the attic
Task 3.3: buy the ingredients
Task 3.2: prepare the cake
Task 3.3: put the cake in the oven and wait till it's done
Task 3.4: finish the cake

The main benefit of this structure is

---?code=rust/async-await-channel/src/lib.rs&lang=rust
@snap[north-east]
## Rust
@snapend

@[1-2]
@[4-18]
@[5-8,12-17]
@[9-11]
@[6]
@[7]
@[13]
@[10]
@[13]
@[15](Hello)
@[16]
@[10]
@[16](World)

---?code=rust/async-await-sieve/src/lib.rs&lang=rust
@snap[north-east]
## Rust
@snapend

@[1-4]
@[6-10]
@[12-24]
@[26-41]

---
@snap[north-west]
## async/await
@snapend

@css[fragment](Rust's **async/await** syntax allows us to express **concurrent design** easily.)

@css[fragment](Note that we did not say anything about how much **parallelism** we want!)

@css[fragment](This code can be run on a **single CPU** but can also make use of **multiple cores** if available.)

---
@snap[north]
## async/await
@snapend

@css[fragment](Rust's **async/await** syntax allows us to express **concurrent design** easily.)

@css[fragment](Note that we did not say anything about how much **parallelism** we want!)

@css[fragment](This code can be run on a **single CPU** but can also make use of **multiple cores** if available.)

---
## async/await

@css[fragment](Rust's **async/await** syntax allows us to express **concurrent design** easily.)

@css[fragment](Note that we did not say anything about how much **parallelism** we want!)

@css[fragment](This code can be run on a **single CPU** but can also make use of **multiple cores** if available.)

---
| 	|GO 	|Rust (thread) 	|Rust (task) 	| 	
|-------|:-----:|:-------------:|:-------------:|
|100 	|2.5 ms	|35 ms 		|4.5 ms 	|
|1000 	|180 ms	|2.25 s		|240 ms 	|

---
CPU usage looks the same

---
Parallelism

pro: throughput
con: synchronisation

---
Synchronisation is expensive

trade-off:
- cost for synchronisation
- gain from parallelism

twice as many workers ?= doubled throughput

---
Concurrency without Parallelism

With only a handful of threads, the concurrent sieve might benefit more from less synchronisation than from parallelism.

---
Concurrency without Parallelism

Visual simulation of the concurrent sieve

---?code=rust/async-await-sieve/src/lib.rs&lang=rust
@snap[north-east]
## Rust
@snapend

@[1-4]
@[26-41]
@[28]
@[38]
@[8,14-17]
@[14-17,20]
@[20,31-34]

---
|executor 	|channel 	| any good? 		| 	
|:-------------:|:-------------:|:---------------------:|
|**mt**		|**sync**	|**yes**		|
|mt		|no sync	|race condition	|
|st 		|sync 		|unnecessary		|
|st 		|no sync	|yes 			|

<br><br>
We already have this!

+++
|executor 	|channel 	| any good? 		| 	
|:-------------:|:-------------:|:---------------------:|
|mt 		|sync 		|yes 			|
|**mt**		|**no sync**	|**race condition**	|
|st 		|sync 		|unnecessary		|
|st 		|no sync	|yes 			|

<br><br>
**Impossible in *safe* Rust!**

+++
|executor 	|channel 	| any good? 		| 	
|:-------------:|:-------------:|:---------------------:|
|mt 		|sync 		|yes 			|
|mt		|no sync	|race condition		|
|**st**		|**sync**	|**unnecessary**	|
|st 		|no sync	|yes 			|

<br><br>
Nonsensical

+++
|executor 	|channel 	| any good? 		| 	
|:-------------:|:-------------:|:---------------------:|
|mt 		|sync 		|yes 			|
|mt		|no sync	|race condition		|
|st 		|sync 		|unnecessary		|
|**st**		|**no sync**	|**yes**		|

<br><br>
There is no such channel in the popular libraries..

---
@snap[north-east]
## Rust
@snapend

```rust
trait Summary {
    fn summarize(&self) -> String;
}

struct Tweet {
	username: String,
	content: String,
}
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}
```
@[1-3]
@[5-8]
@[1-3,5-8,9-14]
@[15-17]

---
@snap[north-east]
## Rust
@snapend

```rust
pub fn spawn<F, T>(future: F) -> JoinHandle<T>
where
    F: Future<Output = T> + Send + 'static,
    T: Send + 'static,
{
    /* Implementation */
}
```

@[3]
@[3,4]

---
## Send

The **Send** @css[text-orange](*marker trait*) indicates that ownership of the type implementing **Send** @css[text-orange](*can be transferred between threads*)

@css[fragment](Almost every Rust type is **Send**)

---
@snap[north-east]
## Rust
@snapend

```rust
pub fn spawn_local<F, T>(future: F) -> JoinHandle<T>
where
    F: Future<Output = T> + 'static,
    T: 'static, 
{
    /* Implementation */
}
```

@[3,4]

<br>
@css[fragment](since async-std v1.6.0)

---

|executor 	|channel 	| any good? 		| 	
|:-------------:|:-------------:|:---------------------:|
|mt 		|sync 		|yes 			|
|mt		|no sync	|race condition		|
|**st**		|**sync**	|**unnecessary**	|
|st 		|no sync	|yes 			|

---
async_std's mpmc channel is @css[text-green](thread-safe)

@ul[list-no-bullets]
- async_std::sync::Sender is **Send** and **Sync**
- async_std::sync::Receiver is **Send** and **Sync**
@ulend

<br><br>
@css[fragment](my spsc channel is @css[text-orange](not thread-safe))

@ul[list-no-bullets]
- spsc::Sender is **!Send** and **!Sync**
- spsc::Receiver is **!Send** and **!Sync**
@ulend

---
## SPSC

My spsc channel is not thread-safe
i.e. it is **!Send** and **!Sync**

---
|executor 	|channel 	|Core2 Quad (1000)	|Core2 Quad (2000)	| 	
|:-------------:|:-------------:|:---------------------:|:---------------------:|
|mt 		|sync 		|240 ms 		|690 ms 		|
|mt		|no sync	|- 			|- 			|
|st		|sync 		|180 ms 		|730 ms 		|
|st 		|no sync	|**90 ms**		|**350 ms** 		|

---
|executor 	|channel 	|E7-4880 4/60/120 (1000)|E7-4880 4/60/120 (2000)| 	
|:-------------:|:-------------:|:---------------------:|:---------------------:|
|mt 		|sync 		|110 ms 		|**155 ms** 		|
|mt		|no sync	|- 			|- 			|
|st		|sync 		|110 ms 		|470 ms 		|
|st 		|no sync	|**60 ms**		|270 ms 		|

---
## Asynchronous Rust libraries
@ul
- async-std
- tokio
- async-task
- futures
@ulend

---
![IMAGE](assets/img/asynchronousRust.png)

---
![IMAGE](assets/img/1000.png)

---
## Punchline

@css[fragment](The fastest concurrent implementation finds the first 1000 primes in)

@css[fragment](**90 ms**)

<br>
@css[fragment](The sequential sieve in)

@css[fragment](**4 ms**)

---
@css[text-12](Async-await debuted on stable Rust in November 2019..)

<br>
@css[fragment](So what did folks do before?)

<br>
@css[fragment text-12](They wrote @css[text-orange](state machines)!)

<br>
@css[fragment](And, by the way, this is what now the Rust compiler does uner the hood as well..)
@css[fragment](it **turns an @css[font-source-sans-pro](async fn) into a normal function returning a state machine**.)

---
## async transform

<br>
```rust
async fn foo() -> Vec<i32> {
	/ *Implementation */
}
```

```rust
fn foo() -> impl Future<Output = Vec<i32>> {
	/ *Implementation */
}
```

@[1,4]

---
## What did I implement? (lib)
@ul
- @css[text-09](an asynchronous signle threaded executor)
- @css[text-09](an asynchronous "non-blocking" single threaded bounded spsc channel)
  - @css[text-07](using Rust's new async/await syntax)
  - @css[text-07](using hadwritten state machines)
- @css[text-09](a "blocking" multi threaded bounded spsc channel)
  - @css[text-07](using @css[font-source-sans-pro](std::sync::Condvar))
  - @css[text-07](using @css[font-source-sans-pro](std::thread::park) and @css[font-source-sans-pro](std::thread::Thread::unpark))
@ulend

---
## What did I implement? (client)

@ul
- @css[text-09](The concurrent sieve algorith)
  - @css[text-08](using the various asynchronous libraries available)
  - @css[text-08](using my own executor and channel)
  - @css[text-08](using handwritten state machines)
  - @css[text-08](using the @css[font-source-sans-pro](std::thread) library and my multi threaded channels)
- @css[text-09](The sequential sieve algorith)

@ulend

---
## What did I implement? (benchmarking)

@ul
- @css[text-09](Benchmarks for Rust with @css[font-source-sans-pro](Criterion))
- @css[text-09](Benchmarks for GO with the official @css[font-source-sans-pro](testing) package)

@ulend

---
## What did I experiment with? (benchmarking)

@ul
- @css[text-09](the Unix @css[font-source-sans-pro](time) command)
- @css[text-09](shell scripting)
- @css[text-09](hyperfine)
- @css[text-09](handwritten benchmarks)
- @css[text-09](tmux (for running long tests on remote machines))
@ulend

---
@snap[north span-100 text-07]
## What the slides do not cover?
@snapend

@snap[west span-50]
@ul
- @css[text-09](Rust's unique features)
  - @css[text-08](ownership and borrowing)
  - @css[text-08](lifetimes)
- @css[text-09](corountines in general)
- @css[text-09](generators in general)
- @css[text-09](state machines in general)
@snapend
@snap[east span-50]
@ul
- @css[text-09](@css[font-source-sans-pro](std::future::Future))
- @css[text-09](@css[font-source-sans-pro](std::pin::Pin))
- @css[text-09](@css[font-source-sans-pro](std::task::Context))
- @css[text-09](@css[font-source-sans-pro](std::task::RawWaker))
- @css[text-09](@css[font-source-sans-pro](std::task::RawWakerVTable))
- @css[text-09](@css[font-source-sans-pro](std::thread_local))
- @css[text-09](@css[font-source-sans-pro](std::ops::Generator))
@ulend
@snapend

---
## Challenges (Benchmarking) 

@css[fragment](correctness)

@css[fragment](extremely time consuming - esp. the longer ones)
  
---
## Axes

@css[fragment](executor)

@css[fragment](channel)

@css[fragment](number of primes)

@css[fragment](capacity)

@css[fragment](CPU)

---
@snap[span-100 text-08]
## Challenges (Duplicate Code) 
@snapend

@ul
  - executor: async-std, channel: async-std
  - executor: async-std, channel: tokio
  - remove duplicate code without affecting the benchmarks
@ulend

---
@snap[span-100 text-06]
## Challenges (Dynamic Ecosystem) 
@snapend

@css[fragment](rough edges)

@css[fragment](unstable features)

@css[fragment](new features)
 
@css[fragment](incomplete documentations)

---
## Axes

@css[fragment](executor)

@css[fragment](channel)

@css[fragment](number of primes)

@css[fragment](capacity)

@css[fragment](CPU)

---
# Fun with Rust

---


























---
................................................................................

---
## async/await under the hood

corountine
generator

std::Future::Future
std::pin::Pin
std::task::Context
std::task::RawWaker
std::task::RawWakerVTable
std::thread_local
std::ops::Generator


---
---
## Axes

@css[fragment](executor)

@css[fragment](channel)

@css[fragment](number of primes)

@css[fragment](capacity)

@css[fragment](CPU)


Clean your place, bake a cake and write a book.


In the computer science world concurrent also means "seemingly parallel" 

This is achieved by extremely rapid task switching.

Concurrent processes can be executed on one core by interleaving the execution steps of each process

---
## Multitasking

Multitasking is the concurrent execution of multiple tasks (Wiki)

A concurrent system is one where a computation can advance without waiting for all other computations to complete.

Concurrent processes can be executed on one core by interleaving the execution steps of each process

Concurrency in the sense of "seemingly parallel" achieved by extremely rapid task switching.

![IMAGE](assets/img/CppConcurrencyInActionCh1Fig1-1TaskSwitching.jpg)

---




Why concurrency?

- throughput
concurrent design makes parallelization easy

- responsiveness

- convenience

---
responsiveness

Often, processes 















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

