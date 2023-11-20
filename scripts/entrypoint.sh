#!/bin/bash

export SUBSTREAMS_API_TOKEN=$(curl https://auth.streamingfast.io/v1/auth/issue -s --data-binary '{"api_key":"'$STREAMINGFAST_KEY'"}' | jq -r .token)

echo $SUBSTREAMS_API_TOKEN
echo "Starting..."

substreams-sink-sql $SINK_MODE $SINK_DB substreams.yaml -e mainnet.sol.streamingfast.io:443 142384016: $SINK_FLAGS

exec "$@"