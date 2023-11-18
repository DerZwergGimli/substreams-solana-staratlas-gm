#!/bin/bash

SUBSTREAMS_API_TOKEN=$(curl https://auth.streamingfast.io/v1/auth/issue -s --data-binary "{"api_key":"$STREAMINGFAST_KEY"}" | jq -r .token)
export SUBSTREAMS_API_TOKEN
substreams-sink-sql run "psql://dev-node:insecure-change-me-in-prod@postgres:5432/dev-node?sslmode=disable" substreams.yaml -e mainnet.sol.streamingfast.io:443 142384016: --final-blocks-only

exec "$@"
