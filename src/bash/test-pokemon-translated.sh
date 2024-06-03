#!/bin/bash

curl -vX GET localhost:8080/api/v1/pokemon/mewtwo/translated | jq .