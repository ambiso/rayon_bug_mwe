#!/usr/bin/env bash

cargo build --release
n=100
out=/tmp/output
failures=0
for ((i=0; i<$n;++i)); do
    cargo run --release > $out 2>/dev/null &
    pid=$!
    sleep 1
    kill $pid
    sleep 0.1
    t="$(grep Expecting $out | sed 's/[^0-9]//g')"
    grep "Running: $t" $out > /dev/null
    failure=$?
    failures=$(($failures + $failure))
    echo Reproduced $failures / $((i + 1)) times
done
