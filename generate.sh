#!/bin/sh


# rm -rf ./gen
protoc -I ./proto/ --plugin=protoc-gen-rust=/mnt/unFSmodern/CLONES/pb-jelly/pb-jelly-gen/codegen/codegen.py --rust_out=./gen ./proto/esphome/api.proto
