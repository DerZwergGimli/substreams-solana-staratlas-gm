#!/bin/bash

SUBSTREAMS_API_TOKEN=$(curl https://auth.streamingfast.io/v1/auth/issue -s --data-binary "{"api_key":"$STREAMINGFAST_KEY"}" | jq -r .token)
export SUBSTREAMS_API_TOKEN
substreams-sink-sql run $DB substreams.yaml -e mainnet.sol.streamingfast.io:443 142384016: --irreversible-only

exec "$@"
