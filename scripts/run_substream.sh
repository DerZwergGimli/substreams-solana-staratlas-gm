#!/usr/bin/bash

source env_init
substreams-sink-sql run $DB substreams.yaml -e mainnet.sol.streamingfast.io:443 142384016: --irreversible-only