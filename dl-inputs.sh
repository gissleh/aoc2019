#!/usr/bin/env sh

. ./cookie.env

alias CURL="curl -H 'authority: adventofcode.com' -H 'dnt: 1' -H 'user-agent: Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/78.0.3904.108 Safari/537.36' -H 'sec-fetch-user: ?1' -H 'accept: text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3' -H 'sec-fetch-site: same-origin' -H 'sec-fetch-mode: navigate' -H 'accept-encoding: gzip, deflate, br' -H 'accept-language: nb,no;q=0.9,nn;q=0.8,en-US;q=0.7,en;q=0.6,de-DE;q=0.5,de;q=0.4' -H 'cookie: session=$AOC_SESSION' --compressed"

mkdir -p input

if [ ! -f "./input/day01.txt" ]; then
  echo "Getting day01 input..."
  CURL https://adventofcode.com/2019/day/1/input -o "./input/day01.txt"
fi

if [ ! -f "./input/day02.txt" ]; then
  echo "Getting day02 input..."
  CURL https://adventofcode.com/2019/day/2/input -o "./input/day02.txt"
fi

if [ ! -f "./input/day03.txt" ]; then
  echo "Getting day03 input..."
  CURL https://adventofcode.com/2019/day/3/input -o "./input/day03.txt"
fi

if [ ! -f "./input/day04.txt" ]; then
  echo "Getting day04 input..."
  CURL https://adventofcode.com/2019/day/4/input -o "./input/day04.txt"
fi
