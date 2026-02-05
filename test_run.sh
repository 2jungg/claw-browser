#!/bin/bash
export PATH="$HOME/.cargo/bin:$PATH"
# Run the browser in Xvfb and take a screenshot after 10 seconds
cargo run &
sleep 10
scrot -z screenshot.png
pkill -f claw_browser
