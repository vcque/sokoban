#!/bin/bash
project=sokoban

clear
rustc src/$project.rc --test --out-dir bin/ -L lib
cd bin
./$project
