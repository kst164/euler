#!/bin/zsh
sed "s/FILE/$1/g" < .problemsTemplate.txt > problems.rs
