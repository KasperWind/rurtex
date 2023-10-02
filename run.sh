#!/bin/bash
# ./maelstrom/maelstrom test -w echo       --bin ~/repos/rust/rurtex/target/release/rurtex --node-count 1 --time-limit 10 --concurrency 16
# ./maelstrom/maelstrom test -w unique-ids --bin ~/repos/rust/rurtex/target/release/rurtex --time-limit 30 --rate 1000 --node-count 3 --availability total --nemesis partition --concurrency 16
./maelstrom/maelstrom test -w broadcast --bin ~/repos/rust/rurtex/target/release/rurtex --time-limit 10 --rate 10 --node-count 1 --concurrency 16
