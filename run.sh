#!/bin/bash
dir_name=$(dirname "$0")

$dir_name/spreadsheet_server &
server_pid=$!
echo $server_pid
until $(nc -z 0.0.0.0 9090)
do
    echo "sleeping"
    sleep 0.5
done
$dir_name/client_app &&
trap "kill $server_pid" EXIT
