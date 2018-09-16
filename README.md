# Toy Language Benchmarking Game

A short benchmark to test out and compare programming languages.

## Motivation and Overview

The main inspiration behind this is Debian's own ["Computer Language Benchmarks Game"](https://benchmarksgame-team.pages.debian.net/benchmarksgame/), but also my own desire to learn as many programming languages as possible and find _"the fastest language"_ (for a particular project).

The main idea behind this game is to use a benchmark that represents a modern, real-world workload which is, primarily, I/O-bound instead of CPU bound. It is, by no means, a replacement for actual CPU-bound benchmarks (like the ones in Debian's site). Just that different things are being tested and showcased in each.

## The Benchmark Game

The program being implement in each language is rather straighforward. Given two directory paths as arguments:

* Scan the two directories (a,b)
* Generate a checksum of all the files in each directory
* Compare the checksums, last modified time and size of the files between directories a & b
* Generate patches to reconcile file changes in both and write it to disk

A patch is a series of actions that will turn one directory into the other. In this case, the game must output two patches that can turn 'a' into 'b' and vice-versa.

## Implementation Details

### The Runner

There's a minimal runner available which is a set of Python 3 scripts to help execute each game and have a way of comparing results. It's fairly simple to use and can get you up an running fairly quickly if you want to (re)implement a language

```
 'init <language name>' to start implementing a new <language>
 'run <language> [space-separated arguments]' to run a given <language> implementation with a set of [arguments]
 'verify <language> [space-separated arguments]' to check a given <language> against the reference
 'benchmark <repetitions> <language> [space-separated arguments]' run an implementation and take an average time
 'compare <comma-separated list of languages> <repetitions> [space-separated arguments]' run some implementations and compare the average time
 'plot/boxplot <comma-separated list of languages> <repetitions> [space-separated arguments]' benchmark and plot the results
 'table <comma-separated list of languages> <repetitions> [space-separated arguments]' benchmark and save a table with the results
 ```

 ### Particulars on Implementation

A couple of details to keep in mind for any implementation of a language in the benchmark:
* Implementations should have MD5, SHA1 and SHA256 as checksums. Adler32 and CRC32 are optional (but recommended)
* A checksum to use can be passed as a flag when calling an implementation (e.g. "--sha1"). If no flag is passed, MD5 is the default checksum.
* The two directories are passed in as positional command line arguments
* File Conflicts should be identified in the patch, but not resolved
* Final output of the program should be in "reference.patch" if you want the runner to verify it

Finally, remember the goal is to **make the fastest program possible** that can complete the benchmark. Some languages might have better support for features for handling I/O workloads and multithreading; take advantage of it. Furthermore:
* Any and all libraries are allowed, as long as they're public and portable (i.e. anyone cloning the repo can find them and install them)
* Multithreading, async I/O and any unique language features are fair game. As long as it runs fast :)
* Any number of threads/processes can be spawned/forked. They should all end when your program does as well
* The process(es) shouldn't use any OS-specific system calls to make itself faster (e.g. 'nice' on *NIX)

## Future Work

This Benchmark Game is *far* from complete. There's a lot of languages missing from this repo (hopefully not for long). Likewise, there's some work to be done on the 'runner' since it can't be used on Windows just yet. And there's a lot of profiling work needed to really squeeze the most performance out of each existing implementation.