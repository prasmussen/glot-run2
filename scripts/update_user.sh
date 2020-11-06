#!/bin/bash

user_id="$1"

curl --request PUT \
     --header 'Authorization: Token tamed-busman-want-vendetta' \
     --header 'Content-type: application/json' \
     --data '{"token": "some-user-token"}' \
     --url "http://localhost:8089/admin/users/$1"
