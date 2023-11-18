#!/bin/bash

SUBSTREAMS_API_TOKEN=$(curl https://auth.streamingfast.io/v1/auth/issue -s --data-binary "{\"api_key\":\"$STREAMINGFAST_KEY\"}" | jq -r .token)

export SUBSTREAMS_API_TOKEN

exec "$@"
