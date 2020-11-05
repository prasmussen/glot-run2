#!/bin/bash

curl --request PUT \
     --header 'Authorization: Token some-secret' \
     --header 'Content-type: application/json' \
     --data '{"name": "bash", "version": "latest", "image": "glot/bash:latest"}' \
     --url 'http://localhost:8089/admin/languages'
