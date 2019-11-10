#!/bin/bash

AVG_REPETITIONS=5

# Average size run
./runner.py boxplot all 5 /media/DMZ{,-1}/Queue/Articles
./runner.py boxplot all 5 /media/DMZ{,-1}/Queue
./runner.py plot all 5 /media/DMZ{,-1}/Queue/Articles
./runner.py plot all 5 /media/DMZ{,-1}/Queue
./runner.py boxplot all 5 /media/DMZ{,-1}/Personal/Projects/\[Done\]/Website
./runner.py plot all 5 /media/DMZ{,-1}/Personal/Projects/\[Done\]/Website

# Heavier runs
./runner.py plot all 3 /media/DMZ{,-1}/Personal/My\ Music
./runner.py plot all 1 /media/DMZ{,-1}/Personal/My\ Pictures
./runner.py plot all 1 /media/DMZ{,-1}/Personal/My\ Video

# Ultimate
./runner.py plot all 1 /media/DMZ/PCs/Virtual/WIN7VR /media/COMDISK-2/PCs/Virtual/WIN7VR

