#!/bin/bash

# start the backend
cd actix-api && cargo run &

# start the Tauri app with yew ui
cd .. && cargo tauri dev