
---
**Command:** `cargo run --bin 1_beagleTA_parser`

**Success:** true

**Output:**
```
[2K
    _█ █ █ █ █________________________________________ █ █________________ █ █ █ █ █ █______ █ █_____
    _█ █____ █ █____ █ █ █____ █ █ █________ █ █ █ █__ █ █______ █ █ █________ █ █________ █ █ █ █___
    _█ █ █ █ █____ █ █ █ █ █______ █ █____ █ █__ █ █__ █ █____ █ █ █ █ █______ █ █______ █ █____ █ █_
    _█ █____ █ █__ █ █________ █ █ █ █______ █ █ █ █__ █ █____ █ █____________ █ █______ █ █ █ █ █ █_
    _█ █ █ █ █______ █ █ █ █__ █ █ █ █ █________ █ █__ █ █ █____ █ █ █ █______ █ █______ █ █____ █ █_
    _______________________________________ █ █ █ █___________________________________________________
    
[2K	 Package name: 'BeagleTA'.
[2K	 Package version: '1.5.11'.
[2K	 Package edition: '2021'.

[2K	 Usage: cargo run <keyword_file.txt> <corpus_dir>
	 🐕  Enter the path to the keyword text file: 	 🐕  Enter the path to the data directory: [2K	 Keyword path cannot be empty!
[2K	 Directory path cannot be empty!

[stderr]:    Compiling proc-macro2 v1.0.94
   Compiling unicode-ident v1.0.18
   Compiling libc v0.2.171
   Compiling crossbeam-utils v0.8.21
   Compiling serde v1.0.219
   Compiling memchr v2.7.4
   Compiling zerocopy v0.8.23
   Compiling rayon-core v1.12.1
   Compiling utf8parse v0.2.2
   Compiling cfg-if v1.0.0
   Compiling anstyle-parse v0.2.6
   Compiling is_terminal_polyfill v1.70.1
   Compiling anstyle v1.0.10
   Compiling colorchoice v1.0.3
   Compiling hashbrown v0.15.2
   Compiling equivalent v1.0.2
   Compiling anstyle-query v1.1.2
   Compiling anstream v0.6.18
   Compiling aho-corasick v1.1.3
   Compiling strsim v0.11.1
   Compiling clap_lex v0.7.4
   Compiling indexmap v2.8.0
   Compiling ryu v1.0.20
   Compiling itoa v1.0.15
   Compiling futures v0.1.31
   Compiling crossbeam-epoch v0.9.18
   Compiling quote v1.0.40
   Compiling crossbeam-deque v0.8.6
   Compiling getrandom v0.2.15
   Compiling rand_core v0.6.4
   Compiling syn v2.0.100
   Compiling iovec v0.1.4
   Compiling num_cpus v1.16.0
   Compiling crossbeam-channel v0.5.15
   Compiling crossbeam-queue v0.3.12
   Compiling byteorder v1.5.0
   Compiling regex-syntax v0.8.5
   Compiling either v1.15.0
   Compiling winnow v0.7.4
   Compiling serde_json v1.0.140
   Compiling rayon v1.10.0
   Compiling crossbeam v0.8.4
   Compiling futures-cpupool v0.1.8
   Compiling bytes v0.4.12
   Compiling ppv-lite86 v0.2.21
   Compiling clap_builder v4.5.35
   Compiling rand_chacha v0.3.1
   Compiling csv-core v0.1.12
   Compiling fs v0.0.5
   Compiling rand v0.8.5
   Compiling colored v3.0.0
   Compiling regex-automata v0.4.9
   Compiling serde_derive v1.0.219
   Compiling jwalk v0.8.1
   Compiling clap v4.5.35
   Compiling regex v1.11.1
   Compiling serde_spanned v0.6.8
   Compiling toml_datetime v0.6.8
   Compiling csv v1.3.1
   Compiling toml_edit v0.22.24
   Compiling toml v0.8.20
   Compiling BeagleTA v1.5.11 (/Users/obonhamcarter/Desktop/mini_research_TA/BeagleTA)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 7.00s
     Running `target/debug/1_beagleTA_parser`
```

**Time:** 7.2878 seconds

---
**Command:** `cargo run --bin 2_csv_cleaner_filter/`

**Success:** false

**Output:**
```
[stderr]: error: no bin target named `2_csv_cleaner_filter/`

	Did you mean `2_csv_cleaner_filter`?
```

**Time:** 0.0611 seconds

---
**Command:** `cargo run --bin 2_csv_cleaner_filter`

**Success:** false

**Output:**
```
[2K

    ___█ █ █ █ █ __█ █ _______________________________________________________________
    _█ █ __________█ █ ______█ █ █ ____█ █ █ ______█ █ █ █ ______█ █ █ ____█ █ __█ █ _
    _█ █ __________█ █ ____█ █ █ █ █ ______█ █ ____█ █ __█ █ __█ █ █ █ █ __█ █ █ █ ___
    _█ █ __________█ █ ____█ █ ________█ █ █ █ ____█ █ __█ █ __█ █ ________█ █ _______
    ___█ █ █ █ █ __█ █ █ ____█ █ █ █ __█ █ █ █ █ __█ █ __█ █ ____█ █ █ █ __█ █ _______
    __________________________________________________________________________________
    
        

[stderr]:    Compiling BeagleTA v1.5.11 (/Users/obonhamcarter/Desktop/mini_research_TA/BeagleTA)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.22s
     Running `target/debug/2_csv_cleaner_filter`
error: the following required arguments were not provided:
  --input <input>
  --output <output>
  --column <column>

Usage: 2_csv_cleaner_filter --input <input> --output <output> --column <column>

For more information, try '--help'.
```

**Time:** 0.4488 seconds

---
**Command:** `cargo run examples/kws.txt examples/corpus_short/`

**Success:** false

**Output:**
```
[stderr]: error: `cargo run` could not determine which binary to run. Use the `--bin` option to specify a binary, or the `default-run` manifest key.
available binaries: 1_beagleTA_parser, 2_csv_cleaner_filter, 3_csv_random_sampler
```

**Time:** 0.0413 seconds

---
**Command:** `cargo run --bin -- 1_beagleTA_parser examples/kws.txt examples/corpus_short/`

**Success:** false

**Output:**
```
[stderr]: error: "--bin" takes one argument.
Available binaries:
    1_beagleTA_parser
    2_csv_cleaner_filter
    3_csv_random_sampler
```

**Time:** 0.0379 seconds

---
**Command:** `cargo run -- --bin 1_beagleTA_parser examples/kws.txt examples/corpus_short/`

**Success:** false

**Output:**
```
[stderr]: error: `cargo run` could not determine which binary to run. Use the `--bin` option to specify a binary, or the `default-run` manifest key.
available binaries: 1_beagleTA_parser, 2_csv_cleaner_filter, 3_csv_random_sampler
```

**Time:** 0.0331 seconds

---
**Command:** `cargo run --bin 1_beagleTA_parser examples/kws.txt examples/corpus_short/`

**Success:** true

**Output:**
```
[2K
    _█ █ █ █ █________________________________________ █ █________________ █ █ █ █ █ █______ █ █_____
    _█ █____ █ █____ █ █ █____ █ █ █________ █ █ █ █__ █ █______ █ █ █________ █ █________ █ █ █ █___
    _█ █ █ █ █____ █ █ █ █ █______ █ █____ █ █__ █ █__ █ █____ █ █ █ █ █______ █ █______ █ █____ █ █_
    _█ █____ █ █__ █ █________ █ █ █ █______ █ █ █ █__ █ █____ █ █____________ █ █______ █ █ █ █ █ █_
    _█ █ █ █ █______ █ █ █ █__ █ █ █ █ █________ █ █__ █ █ █____ █ █ █ █______ █ █______ █ █____ █ █_
    _______________________________________ █ █ █ █___________________________________________________
    
[2K	 Package name: 'BeagleTA'.
[2K	 Package version: '1.5.11'.
[2K	 Package edition: '2021'.

[2K	 Usage: cargo run <keyword_file.txt> <corpus_dir>
[2K	 Found 63 files in the directory 'examples/corpus_short/'.
[2K	 Key words : ["2024", "ethic", "bio", "machine", "mitochondria"]
[2K 	 * PMC314466.txt -> 1 of 63[2K 	 * PMC314300.txt -> 2 of 63[2K 	 * PMC314472.txt -> 3 of 63[2K 	 * PMC193604.txt -> 4 of 63[2K 	 * PMC193605.txt -> 5 of 63[2K 	 * PMC314473.txt -> 6 of 63[2K 	 * PMC314467.txt -> 7 of 63[2K 	 * PMC314301.txt -> 8 of 63[2K 	 * PMC314471.txt -> 9 of 63[2K 	 * PMC314465.txt -> 10 of 63[2K 	 * PMC193607.txt -> 11 of 63[2K 	 * PMC193606.txt -> 12 of 63[2K 	 * PMC314464.txt -> 13 of 63[2K 	 * PMC314470.txt -> 14 of 63[2K 	 * PMC314474.txt -> 15 of 63[2K 	 * PMC314475.txt -> 16 of 63[2K 	 * PMC314463.txt -> 17 of 63[2K 	 * PMC314477.txt -> 18 of 63[2K 	 * PMC314476.txt -> 19 of 63[2K 	 * PMC314462.txt -> 20 of 63[2K 	 * PMC212706.txt -> 21 of 63[2K 	 * PMC300882.txt -> 22 of 63[2K 	 * PMC212699.txt -> 23 of 63[2K 	 * PMC212698.txt -> 24 of 63[2K 	 * PMC300883.txt -> 25 of 63[2K 	 * PMC212705.txt -> 26 of 63[2K 	 * PMC300881.txt -> 27 of 63[2K 	 * PMC212704.txt -> 28 of 63[2K 	 * PMC176548.txt -> 29 of 63[2K 	 * PMC212700.txt -> 30 of 63[2K 	 * PMC300884.txt -> 31 of 63[2K 	 * PMC300700.txt -> 32 of 63[2K 	 * PMC300885.txt -> 33 of 63[2K 	 * PMC300675.txt -> 34 of 63[2K 	 * PMC212701.txt -> 35 of 63[2K 	 * PMC212703.txt -> 36 of 63[2K 	 * PMC212688.txt -> 37 of 63[2K 	 * PMC212689.txt -> 38 of 63[2K 	 * PMC300886.txt -> 39 of 63[2K 	 * PMC212702.txt -> 40 of 63[2K 	 * PMC212690.txt -> 41 of 63[2K 	 * PMC212691.txt -> 42 of 63[2K 	 * PMC261870.txt -> 43 of 63[2K 	 * PMC212687.txt -> 44 of 63[2K 	 * PMC212693.txt -> 45 of 63[2K 	 * PMC212692.txt -> 46 of 63[2K 	 * PMC176545.txt -> 47 of 63[2K 	 * PMC212319.txt -> 48 of 63[2K 	 * PMC212696.txt -> 49 of 63[2K 	 * PMC212697.txt -> 50 of 63[2K 	 * PMC176546.txt -> 51 of 63[2K 	 * PMC212695.txt -> 52 of 63[2K 	 * PMC212694.txt -> 53 of 63[2K 	 * PMC176547.txt -> 54 of 63[2K 	 * PMC314478.txt -> 55 of 63[2K 	 * PMC314479.txt -> 56 of 63[2K 	 * PMC314469.txt -> 57 of 63[2K 	 * PMC314482.txt -> 58 of 63[2K 	 * PMC314468.txt -> 59 of 63[2K 	 * PMC314481.txt -> 60 of 63[2K 	 * PMC314480.txt -> 61 of 63[2K
	 ✨ Run commands to create virtual environment with Python for the following commands ...
[2K	 python3 -m venv ~/Desktop/venv
	 source ~/Desktop/venv/bin/activate
	 pip install plotly matplotlib numpy pandas networkx seaborn scikit-learn pyvis

[2K	 ✨ Run commands to create keyword analysis and other visualizations ...
[2K	 cd 0_out && python3 visualizations.py && cd ..
[2K	 cd 0_out && python3 interactive_network_1.py --datafile output.csv --threshold 6 && cd .. 
[2K	 cd 0_out && python3 interactive_network_2.py --datafile output.csv && cd .. 
[2K	 cd 0_out && python3 complete_network.py -f output.csv -t 6 -nc blue -ec green  && cd .. 


[stderr]:     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.03s
     Running `target/debug/1_beagleTA_parser examples/kws.txt examples/corpus_short/`
```

**Time:** 0.5581 seconds
