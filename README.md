# rust_crawler_comparison
compare async impl for simple crawler

# urls
get 10 urls from [common crawl](https://commoncrawl.org/)

# methods

| dir           | method                                                                                                                                                               | time     |
| ------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------- | -------- |
| sequential    | for loop + ureq                                                                                                                                                      | 13-14s   |
| multithreaded | spawn 10 thread                                                                                                                                                      | 2.1-2.6s |
| async_tokio   | Tokio+ureq                                                                                                                                                           | 3.2-4.3  |
| async_tokio_2 | AI rewrite from [zjp-CN](https://github.com/zjp-CN) [impl](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=404a64695ff36c37b18e4e3d92f849d1) | 1.5-2.5  |
| async_tokio_3 | Tokio+reqwest                                                                                                                                                        | 1.4-2.3  |
# result
效率上，**异步非阻塞>多线程>异步阻塞>顺序执行**
In terms of efficiency: **Asynchronous non-blocking > Multithreading > Asynchronous blocking > Sequential execution.**

在异步中，创建的子线程的数量是不确定的，且本实验中子线程数量越多，运行时间越长
In the asynchronous setup, the number of worker threads created by the runtime is not fixed. In this experiment, it was observed that a higher number of effective concurrent operations (related to how tasks map to threads or how blocking tasks are handled) led to longer runtimes in some async scenarios.

在异步中，存在密集的内核态调用区域，看调用是CA认证，在多线程中可以看到每个线程都有各自的这部分，异步中集中运行
In the asynchronous execution, there is a dense region of kernel-mode calls, observed to be related to CA certification. In the multithreaded setup, each thread independently handles these CA certification calls. In the asynchronous setup, these calls appear to be processed more centrally or concurrently on the async runtime's worker threads.

# some diagram from [samply](https://github.com/mstange/samply)

multi-thread: 10 threads
![multi-thread](/assets/image.png)

dense kernel call: orange color and two worker only
![async](/assets/Pasted%20image%2020250525192537.png)

spawn ureq(should use spawn_blocking for block task): 10 worker
![spawn block](/assets/Pasted%20image%2020250525134918.png)