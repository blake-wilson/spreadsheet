#!/bin/bash
./target/debug/spreadsheet_server &
server_pid=$!
echo $server_pid
until $(nc -z 0.0.0.0 9090)
do
    echo "sleeping"
    sleep 0.5
done
./target/debug/client_app &&
trap "kill $server_pid" EXIT
