## Projects

Directory | Contents
------------ | -------------
[aoc](aoc) | AoC engine, Rust library shared among all executables (listed below).
[year2015](year2015) | Year 2015 puzzle solutions.
[year2016](year2016) | Year 2016 puzzle solutions.
[year2017](year2017) | Year 2017 puzzle solutions.
[year2018](year2018) | Year 2018 puzzle solutions.
[year2019](year2019) | Year 2019 puzzle solutions.
[year2020](year2020) | Year 2020 puzzle solutions.
[year2021](year2021) | Year 2021 puzzle solutions.


## Special files

File | Contents
------------ | -------------
[Cargo.toml](Cargo.toml) | Rust workspace file.


## Build and run projects

- download the repository
- execute command `cargo build --release` to update crates index, download external crates and build all projects
- run selected project using `cargo`: execute command `cargo run --release --bin <project_name>`
- run standalone `exe` file from command line by finding it in *target/release* subdirectory

Note: Each `exe` file will try to find *input* directory, where puzzles' input files are placed. It will happen no matter if you run directly from *target/release* subdirectory or using `cargo` (for details see *load_input()* and *dir_prefix()* functions in [aoc/src/lib.rs](aoc/src/lib.rs) file).


## Command line options

To get familiar with command line options, run the executable with *-h* switch (help):
```sh
> year2021 -h
```

To run single day puzzle solution(s) use *-p* switch and select the day. Example of running 7th day puzzle solution(s):
```sh
> year2021 -p 7
```

Some puzzles have more than one solution of the problem. To see how many solutions is available use *-a* switch:
```sh
> year2020 -a
```

To run single solution also use *-p* switch and select the day and the solution. Example of running 18th day, 2nd puzzle solution:
```sh
> year2020 -p 18:2
```

To run single day solution(s) with your input, place your input file in *bin* directory and put the filename in *-i* switch. Example:
```sh
> year2020 -p 2 -i my_2021_02_input.txt
```

To measure execution time of the solution(s) use *-s* switch. In this mode each puzzle solution is run at least ten times and at least for specified seconds (ten seconds, if numeric value is not provided after *-s* switch). It may take some time to obtain all results, so please be patient. 10% of the highest and 10% of the lowest time measurements are dropped, the average time of all remaining executions is printed. Repeatability of results is checked after each execution. These consistency checks and preparing input data for subsequent code executions are outside the measurement scope, thus the execution may last longer than expected. Time of execution does not include input file loading, but includes processing input data (from vector of strings to any other structure needed by solution). Command example:
```sh
> year2021 -p 2 -s 15
```

Output is printed on console using colored text. Selecting the colors is achieved by emitting escape sequences. This feature is supported by *cmd.exe* and *conhost.exe* processes starting from Windows 10 TH2 v1511. If you see garbage instead of colors, use *-c* switch to disable coloring (it is also useful when redirecting output to file for further processing):
```sh
> year2021 -p 2 -c
```

## Debugging the solution?

Nope. What for? ;)
