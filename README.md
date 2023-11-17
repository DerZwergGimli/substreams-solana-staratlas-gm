# Dev

### Update Proto Buffer

- `substreams protogen substreams.yaml --exclude-paths="sf/substreams,google"`

# Run

1. `docker-compose up `
2. `substreams-sink-sql setup "psql://dev-node:insecure-change-me-in-prod@127.0.0.1:5432/dev-node?sslmode=disable" substreams.yaml `
3. `substreams-sink-sql run "psql://dev-node:insecure-change-me-in-prod@127.0.0.1:5432/dev-node?sslmode=disable" substreams.yaml -e mainnet.sol.streamingfast.io:443 222797230: --irreversible-only`