#!/bin/bash

docker run -it --rm     \
  -v $PWD:/workspace    \
  -w /workspace         \
  rust:1.65.0-bullseye  \
  bash