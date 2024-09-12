# Commit-Challenge
This project aims to automate the creation of commits in Git in order to maximize the number of commits generated per second. The implementation was done in Rust to take advantage of its high performance.


## Overview
The project was inspired by a [repository](https://github.com/virejdasani/Commited) that managed to generate 3 million commits in Python in three days. The solution was reimplemented in Rust due to its efficiency. However, even with Rust, performance was still not optimal, so additional optimizations were made.


## Solution
The current solution uses multiple threads to generate commits simultaneously. The code uses Rust's standard thread library to create and manage multiple threads that execute the `git commit` command in parallel.

### Code structure
- **Number of Threads**: 4
- **Main Function:** Creates a shared variable to count the number of commit and spawn threads that execute the `git commit` command with incremental messages.
- **Synchronization Mechanism**: Uses `Arc` and `Mutex` to ensure that the commit counter is updated safely between threads.


## Hardware Used
### CPU
- **Model:** Intel Core i5-2467M
- **Architecture:** x86_64
- **Number of Cores:** 2
- **Number of Threads:** 4
- **Maximum Clock:** 1.60 GHz
- **Important Flags:** AVX, SSE, HT (Hyper-Threading)

### RAM Memory
- **Total Installed:** 8 GB (2 x 4 GB)
- **Type:** DDR3
- **Speed:** 1333 MT/s
- **Form Factor:** SODIMM

### Storage
- **SSD model:** Toshiba VX500
- **Capacity:** 512 GB
- **Type:** SSD (SATA III)

### Operating System
- **Distribution:** Debian 12


## Results
During the test period, which began at approximately 02:00 and ended at 11:10 the same day, the total number of commits generated was 3,166,923.
- **Average Commits per Second:** Approximately 95.39
- **Average Commits per Hour:** Approximately 343.404