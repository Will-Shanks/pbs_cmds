# PBS commands
- CLI for interacting with pbs job scheduler

## Resource stat
- can stat any type of resource (job, host, que, etc)
- can filter by different attributes, and their values
  - currently can filter by, if an attrib is set, if it is eq, or not equal
  - plan on implementing <, >, <=, and >=
- can filter by name, this uses nodesets
  - need to be exact, ex: `./pbs_cmds host stat casper[2-7]`
  - plan on making this smarter, so things like `./pbs_cmds host stat casper*` will work

## Job submission
- different systems require differnt attributes
- NCAR requires a que, account, job name, walltime, and select
- minimal example: `./pbs_cmds job sub -s ../../benchmarks/stream/stream.sh -q casper Account_Name=SSSG0001 Job_Name=test Resource_List.walltime=1:00:00 Resource_List.select=ncpus=1:ompthreads=1`
