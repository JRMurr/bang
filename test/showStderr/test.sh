#!/usr/bin/env bash


for (( ; ; ))
do
   echo "stdout"
    >&2 echo "error"
    sleep 0.1
done
