#!/bin/bash

curl --request POST \
     --header 'Authorization: Token some-secret' \
     --header 'Content-type: application/json' \
     --data '{"token": "d11088bc-a29d-4d49-a633-b1b1ae807064"}' \
     --url 'http://localhost:8089/admin/users'
