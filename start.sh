#!/bin/bash

for i in "$@" ; do
    if [[ $i == "reset" ]] ; then
        rm -r /tmp/near-sandbox
        break
   fi
done

target/debug/near-sandbox --home /tmp/near-sandbox init && \
target/debug/near-sandbox --home /tmp/near-sandbox run
