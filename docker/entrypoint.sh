#!/bin/sh

FLAG=$(cat /var/flags/flag.txt || exit 0)

if [ ! -z "$FLAG" ]; then
  export HAXAGON_FLAG="$FLAG"
fi

/bin/app