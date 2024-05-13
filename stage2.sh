#! /bin/bash
XAUTHTOKEN=$(cat xauth.txt)
echo "xauth add $XAUTHTOKEN" > xauth.sh
chmod +x xauth.sh
./xauth.sh
cd /AnotherOSbutinrust
. "$HOME/.cargo/env" && cargo run