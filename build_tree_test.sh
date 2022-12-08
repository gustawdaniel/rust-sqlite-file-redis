#! /bin/sh

testEquality() {
  cargo run --bin prepare-tree
  wc collins-scrabble-2019.txt | cut -d' ' -f2,4 > wc1
  wc collins-scrabble-2019-tree.txt | cut -d' ' -f2,4 > wc2
  RES=$(diff wc1 wc2)
  EXP=""
  assertEquals "${EXP}" "${RES}"
}

# Load shUnit2.
. /usr/share/shunit2/shunit2