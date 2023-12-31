#!/bin/bash

export SUBSTREAMS_API_TOKEN=$(curl https://auth.streamingfast.io/v1/auth/issue -s --data-binary '{"api_key":"'$STREAMINGFAST_KEY'"}' | jq -r .token)

echo $SUBSTREAMS_API_TOKEN
echo "Starting..."

substreams-sink-sql run substreams.clickhouse.yaml -e mainnet.sol.streamingfast.io:443 142384016: --irreversible-only

exec "$@"