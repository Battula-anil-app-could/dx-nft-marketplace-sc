#!/bin/sh

cargo install dharitri-sc-meta

TARGET_DIR=$PWD/target

sc-meta all build --target-dir $TARGET_DIR
