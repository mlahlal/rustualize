#!/bin/bash

cargo build && clear && sudo ./target/debug/rustualize -d -u 0 -m /home/mouad/Documents/dev/rustualize/containerdisk -c "/bin/bash"

# sudo debootstrap stable /stable-chroot http://deb.debian.org/debian/
