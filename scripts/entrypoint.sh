#!/bin/bash

export SUBSTREAMS_API_TOKEN=$(curl https://auth.streamingfast.io/v1/auth/issue -s --data-binary '{"api_key":"'$STREAMINGFAST_KEY'"}' | jq -r .token)


echo "Starting..."

substreams-sink-sql $SINK_MODE $DSN $SINK_YAML 142384016: -e mainnet.sol.streamingfast.io:443 --irreversible-only

exec "$@"