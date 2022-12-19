#!/usr/bin/env bash
# -*- coding: utf-8 -*-

# Copyright (c) 2022 Lorenzo Carbonell <a.k.a. atareao>

# Permission is hereby granted, free of charge, to any person obtaining a copy
# of this software and associated documentation files (the "Software"), to deal
# in the Software without restriction, including without limitation the rights
# to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
# copies of the Software, and to permit persons to whom the Software is
# furnished to do so, subject to the following conditions:

# The above copyright notice and this permission notice shall be included in
# all copies or substantial portions of the Software.

# THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
# IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
# FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
# AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
# LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
# OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
# SOFTWARE.

#!/bin/bash

# Used in Docker build to set platform dependent variables

case $TARGETARCH in

    "amd64")
        echo "x86_64-unknown-linux-musl" > /.platform
        echo "" > /.compiler 
        echo "x86_64-linux-gnu" > /.libdir
        ;;
    "arm64") 
        echo "aarch64-unknown-linux-musl" > /.platform
        echo "gcc-aarch64-linux-gnu" > /.compiler
        echo "aarch64-linux-gnu" > /.libdir
        ;;
    "arm")
        export OPENSSL_LIB_DIR="/usr/lib/arm-linux-gnuabihf"
        echo "armv7-unknown-linux-gnueabihf" > /.platform
        echo "gcc-arm-linux-gnueabihf" > /.compiler
        echo "arm-linux-gnueabihf" > /.libdir
        ;;
esac


