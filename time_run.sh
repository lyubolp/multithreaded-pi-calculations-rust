#!/bin/bash

for turn in $(seq 1 $3)
do
  result=$(/usr/bin/time -f "%E" ./target/release/multithreaded_pi_rust -e $1 -t $2 2>&1 > /dev/null);
  echo $result
done
