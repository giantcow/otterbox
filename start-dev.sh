#!/bin/env bash

tmux new -s otterbox \; \
    send-keys 'RUST_LOG=debug cargo run --bin ob_server' C-m \; \
    split-window -v \; \
    send-keys 'cd ob-ui && npm run dev' C-m \; \
    split-window -v
