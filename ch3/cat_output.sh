#!/bin/zsh
# run from project rust_command_line
cat ch3/tests/inputs/empty.txt
echo "\n"
cat ch3/tests/inputs/fox.txt
echo "\n"
# -n|--number and -b|--number-nonblank flags both number the lines
# line numbers are right-justified in a field of 6 characters wide followed by a tab character
# -t to display nonprinting character so that tab shows as ^I
cat -n ch3/tests/inputs/fox.txt | cat -t
echo "\n"
cat -n ch3/tests/inputs/spider.txt

echo "\n"
cat -n ch3/tests/inputs/the-bustle.txt

echo "\n"
# -b blank line is not numbered but stilled printed
cat -b ch3/tests/inputs/the-bustle.txt
