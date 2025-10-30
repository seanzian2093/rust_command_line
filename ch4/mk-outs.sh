#!/bin/zsh
# run from rust_command_line
#INPUTS="ch4/tests/inputs"
#OUT_DIR="ch4/tests/expected"

# run from rust_command_line/ch4 - aligned with tests
INPUTS="./tests/inputs"
OUT_DIR="./tests/expected"

[[ ! -d "$OUT_DIR" ]] && mkdir -p "$OUT_DIR"

for FILE in $INPUTS/*.txt; do
    BASENAME=$(basename "$FILE")
    head      $FILE > ${OUT_DIR}/${BASENAME}.out
    head -n 2 $FILE > ${OUT_DIR}/${BASENAME}.n2.out
    head -n 4 $FILE > ${OUT_DIR}/${BASENAME}.n4.out
    head -c 1 $FILE > ${OUT_DIR}/${BASENAME}.c1.out
    head -c 2 $FILE > ${OUT_DIR}/${BASENAME}.c2.out
    head -c 4 $FILE > ${OUT_DIR}/${BASENAME}.c4.out
done
#  In `zsh`, parameter expansion does NOT perform word splitting by default, so `head $ALL` treats the entire string of paths as one single filename containing spaces, leading to a “no such file or directory” error.
# Make ALL an array in zsh
ALL=(
  $INPUTS/empty.txt
  $INPUTS/one.txt
  $INPUTS/two.txt
  $INPUTS/three.txt
  $INPUTS/twelve.txt
)
head      $ALL > $OUT_DIR/all.out
head -n 2 $ALL > $OUT_DIR/all.n2.out
head -n 4 $ALL > $OUT_DIR/all.n4.out
head -c 1 $ALL > $OUT_DIR/all.c1.out
head -c 2 $ALL > $OUT_DIR/all.c2.out
head -c 4 $ALL > $OUT_DIR/all.c4.out
