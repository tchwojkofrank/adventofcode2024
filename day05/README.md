# Day 5: Print Queue

## Intro

First section defines ordering of "pages"
Second section is a list of "instruction updates" that may or may not be correctly ordered pages.

## Part 1

Get the sum of middle page of all the correctly ordered instruction updates.
Basically check that for each pair A,B, that there is a page ordering rule A|B.

## Part 2

For the incorrectly ordered pages, create a custom ordering function using the rules, and then sort using that rule.

## Other notes

I didn't have to translate each page into a number. I should have left them all as strings, and then defined a custom ordering function, and just used sort.

## ChatGPT

ChatGPT got both parts today.