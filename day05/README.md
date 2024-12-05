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

For part 1, I don't think I need to check every pair. If something was out of order, then one of the checks would fail.

## ChatGPT

ChatGPT got both parts today.