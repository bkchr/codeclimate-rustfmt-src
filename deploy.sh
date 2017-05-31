#!/usr/bin/env bash

GIT_COMMIT_MSG_HEADER="New rustfmt engine update"
GIT_COMMIT_MSG_CONTENT="Update based on commit https://github.com/bkchr/codeclimate-rustfmt-src/commit/$(git rev-parse HEAD)"

git clone git@github.com:bkchr/codeclimate-rustfmt.git codeclimate-rustfmt
cd codeclimate-rustfmt

mkdir -p bin

cp -f ../bin/rustfmt bin/
cp -f ../bin/codeclimate-rustfmt bin/

cp -f ../Dockerfile .
cp -f ../engine.json .
cp -f ../main.sh .
cp -f ../install-rust.sh .

git add .
if ! git status -s
then
    git commit -am "$GIT_COMMIT_MSG_HEADER" -m "$GIT_COMMIT_MSG_CONTENT"
    git push
fi

exit 0
