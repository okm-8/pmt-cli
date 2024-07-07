# PMT
PMT is a project for fun. 
This tool use math to make some fun things.
For example, you can use this tool to make a decision
what to eat today.

# Requirements
- Rust

## How to use
### Install
https://www.rust-lang.org/tools/install
```bash
make release
```
This command will generate a binary file in the `bin` directory.

### Run
```bash
./bin/pmt <command> <args>
```

### Help
```bash
./bin/pmt help
./bin/pmt --help
./bin/pmt -h
./bin/pmt help <command>
./bin/pmt <command> --help
./bin/pmt <command> -h
```
You can get help information for any command. 

### Command
#### Choose number
```bash
./bin/pmt choose number \
  --rolls <rolls> \           # how many times to roll the dice for each number
  --count <count> \           # how many numbers to get in the end
  --precision <precision> \   # how many decimal places to keep (be careful, it may cause performance issue)
  {}
```

#### Choose value
```bash
./bin/pmt choose variant \
    --rolls <rolls> \         # how many times to roll the dice for each value
    --count <count> \         # how many values to get in the end
    <value1> <value2> ...     # values to choose from
```

you also can use $(cat <file>) to input values from a file.
```bash
./bin/pmt choose variant \
    --rolls <rolls> \         # how many times to roll the dice for each value
    --count <count> \         # how many values to get in the end
    $(cat <file>)
```

or even keep in file the full configuration
```file
--rolls <rolls>  # how many times to roll the dice for each value
--count <count>  # how many values to get in the end
# values to choose from
VALUE1 
VALUE2 
...         
```
```bash
./bin/pmt choose variant $(cat <file>)
```

### Math
```bash
./bin/pmt math ...
```

It also contains some math functions, like 
different mean (arithmetic, geometric, ...), 
median, mode, variance, standard deviation, etc.
They are being added by the mood.