#!/bin/bash

curl --request PUT \
     --header 'Authorization: Token some-secret' \
     --header 'Content-type: application/json' \
     --data '{"name": "erlang", "version": "latest", "image": "glot/erlang:latest"}' \
     --url 'http://localhost:8089/admin/languages'
