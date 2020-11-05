#!/bin/bash

curl --request POST \
     --header 'Authorization: Token some-secret' \
     --header 'Content-type: application/json' \
     --data '{"files": [{"name": "bash.sh", "content": "echo 42"}]}' \
     --url 'http://localhost:8089/languages/bash/latest'
