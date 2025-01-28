#!/bin/bash

cargo build && clear && sudo ./target/debug/rustualize -d -u 0 -m /rustualize -c "/bin/bash"
