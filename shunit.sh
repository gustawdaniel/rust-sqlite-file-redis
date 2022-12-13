#! /bin/sh

testBuildTree() {
  cargo run --bin prepare-tree
  wc collins-scrabble-2019.txt | cut -d' ' -f2,4 > wc1
  wc collins-scrabble-2019-tree.txt | cut -d' ' -f2,4 > wc2
  RES=$(diff wc1 wc2)
  EXP=""
  assertEquals "${EXP}" "${RES}"
}

testBuildTree5() {
  cargo run --bin prepare-tree -- 5
  assertEquals "5" "$(cat 5-tree.meta)"
}

testBuildBin5() {
  cargo run --bin prepare-bin -- 5
  assertEquals "h=0x00000005,w=0x02" "$(head -n 1 5-bin)"
}

testBuildSqlite() {
  cargo run --bin prepare-sqlite -- 5
  assertEquals "3a" "$(sqlite3 5.db "SELECT * FROM words WHERE word='3a'")"
  assertEquals "" "$(sqlite3 5.db "SELECT * FROM words WHERE word='6'")"
}

testBuildRedis() {
  cargo run --bin prepare-redis -- 5
  assertEquals "1" "$(redis-cli -h "${REDIS_HOST:-127.0.0.1}" SISMEMBER word:5 5)"
  assertEquals "0" "$(redis-cli -h "${REDIS_HOST:-127.0.0.1}" SISMEMBER word:5 6)"
}

testText() {
  assertEquals "true" "$(METHOD="text" cargo run -- 5 5 | tail -n 1)"
  assertEquals "false" "$(METHOD="text" cargo run -- 6 5 | tail -n 1)"
}

testSplit() {
  assertEquals "true" "$(METHOD="split" cargo run -- 5 5 | tail -n 1)"
  assertEquals "false" "$(METHOD="split" cargo run -- 6 5 | tail -n 1)"
}

testBin() {
  assertEquals "true" "$(METHOD="bin" cargo run -- 5 5 | tail -n 1)"
  assertEquals "false" "$(METHOD="bin" cargo run -- 6 5 | tail -n 1)"
}

testMem() {
  assertEquals "true" "$(METHOD="mem" cargo run -- 5 5 | tail -n 1)"
  assertEquals "false" "$(METHOD="mem" cargo run -- 6 5 | tail -n 1)"
}

testSqlite() {
  assertEquals "true" "$(METHOD="sqlite" cargo run -- 5 5 | tail -n 1)"
  assertEquals "false" "$(METHOD="sqlite" cargo run -- 6 5 | tail -n 1)"
}

testRedis() {
  assertEquals "true" "$(METHOD="redis" cargo run -- 5 5 | tail -n 1)"
  assertEquals "false" "$(METHOD="redis" cargo run -- 6 5 | tail -n 1)"
}

# Load shUnit2.
. /usr/share/shunit2/shunit2