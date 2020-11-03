#!/bin/bash

user_id="$1"

curl --request PUT \
     --header 'Authorization: Token some-secret' \
     --header 'Content-type: application/json' \
     --data '{"token": "new-user-token"}' \
     --url "http://localhost:8089/admin/users/$1"
