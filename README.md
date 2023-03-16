# PBS commands
- CLI for interacting with pbs job scheduler

## Resource stat
- can stat any type of resource (job, host, que, etc)
- can filter by different attributes, and their values
  - currently can filter by, if an attrib is set, if it is eq, or not equal
  - plan on implementing <, >, <=, and >=
- can filter by name, this uses nodesets
  - ex: `./pbs_cmds host stat casper[2-7]` matches casper2, casper3, ... casper7
  - ex: `./pbs_cmds host stat gu00[0-1]2` matches gu0002 and gu00012
  - checks given name with start of returned nodes e.g. `./pbs_cmds host stat gu` will match all gust nodes

## Job submission
- different systems require differnt attributes
- NCAR requires a que, account, job name, walltime, and select
- minimal example: `./pbs_cmds job sub -s ../../benchmarks/stream/stream.sh -q casper Account_Name=SSSG0001 Job_Name=test Resource_List.walltime=1:00:00 Resource_List.select=ncpus=1:ompthreads=1`

## Reservation submission
- minimal example: `./pbs_cmds resv sub Reserve_Name=shanks-test reserve_start=1678838400 reserve_duration=30000`
