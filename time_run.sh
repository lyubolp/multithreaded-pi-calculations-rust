#!/bin/bash

for turn in $(seq 1 $3)
do
  result=$(/usr/bin/time -f "%E" ./target/release/multithreaded_pi_rust -p $1 -t $2 2>&1 > /dev/null);
  echo $result
done
