_______________________________________________________________________________

concurrency = managing tasks simulataneously. These task may or may not
be running at the same time. Concurrency involves tasks switching.
Making some progress on a task, 
then pausing and switching another task and the cycle continues until all
tasks have been completed.

Concurrency is required for creating real world applications, 
because there are situations where it is simply impractical for the whole
program to pause and wait for another part of the program to continue.

In programming concurrency (the task switching) happens rapidly so it creates
the illusion that these tasks are all progressing at the same time.

Concurrency is different but related to concepts like parallelism 
and asynchronus programming.

The term concurrency is also refered to as multi-threading
_______________________________________________________________________________

parallelism = simulataneous execution of tasks

This when all tasks are being executed at the same time. 
E.g. multiple workers, dedicated to the completion of each task.

parallelism requires a CPU with multiple cores. Each cores acts as a worker.

So parallelism is about the division of work.

_______________________________________________________________________________

asynchronus = independent task execution
Some tasks runs separately from the main thread. E.g. API calls.

Mechanisms

threads

messaging channels (communication between threads)

shared state - mutex (mutal exclusion lock) 
and atomic types (multiple parts of the program reading
and writing to the same data)

In Rust mutexes are smart. So when a mutex goes out of scope, 
it automatically releases its lock.

Atomic types = Used when you need to perform simpile operations 
on shared data without locking.
_______________________________________________________________________________

concurrent code should be safe and easy to understand

_______________________________________________________________________________

### Issues to watch out for

data races and dead locks

Rust's concurrency model ensures that data is only owned by one thread 
at a time.

data races: when threads interact with data in an unpredictable sequence,
leading to inconsistent outcomes. Two parts of the program racing to access
the same resource.

deadlock: two threads end up waiting for each other indefinately.
_______________________________________________________________________________
