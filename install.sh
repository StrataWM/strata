#!/bin/sh
cargo install --path .;

cd ~/.cache
git clone https://github.com/stratawm/stratactl.git;

cd stratactl;
cargo install --path .;

cd ~/.cache
rm -rf stratactl
exit;