# factorize
Command line tool for factorizing integers

# Usage
```
USAGE:
    factorize [FLAGS] [OPTIONS] <numbers>...

FLAGS:
        --assert     Set to 'true' to assert correctness of factorizations
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --alg <alg>    Algorithm to use for factorization [default: trial_division]

ARGS:
    <numbers>...    Array of numbers to factorize
```

# Example
```
$ factorize --alg brents_rho 543895739 78734 5890654
543895739 => [17, 31993867], took 68.3µs
78734 => [2, 39367], took 20.5µs
5890654 => [2, 7, 11, 29, 1319], took 46.9µs
```
