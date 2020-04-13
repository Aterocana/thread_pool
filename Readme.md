# Thread Pool

A Thread Pool struct example from The Rust Programming Language book.
It's used here with a slightly different main, which compares a total sleep time splitted across a variable number of workers.

As expected, when the number of workers is greater than the number of sleep time, there's no gain of performance (elapsed time =~ number of workers where cardinaly >= limit).