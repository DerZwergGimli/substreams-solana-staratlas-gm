ENDPOINT ?= mainnet.eth.streamingfast.io:443
ENDPOINT_SOL ?= mainnet.sol.streamingfast.io:443
START_BLOCK ?= 12292922
STOP_BLOCK ?= +10

.PHONY: build
build:
	cargo build --target wasm32-unknown-unknown --release

.PHONY: run
run: build
	substreams run -e $(ENDPOINT) substreams.yaml map_block -s $(START_BLOCK) -t $(STOP_BLOCK)


.PHONY: map_market_instructions
map_market_instructions: build
	substreams run -e $(ENDPOINT_SOL) substreams.yaml map_market_instructions -s 222855459 -t 222855461

.PHONY: map_market_instructions_long
map_market_instructions_long: build
	substreams run -e $(ENDPOINT_SOL) substreams.yaml map_market_instructions -s 222797230 -t 222798333


.PHONY: map_market_instructions_from_284817250
map_market_instructions_from_284817250: build
	substreams run -e $(ENDPOINT_SOL) substreams.yaml map_market_instructions -s 284817249 -t 284817251


.PHONY: map_market_instructions_from_XXX
map_market_instructions_from_XXX: build
	substreams run -e $(ENDPOINT_SOL) substreams.yaml map_market_instructions -s 253396016 -t 253396018


.PHONY: map_market_instructions_from_XXX2
map_market_instructions_from_XXX2: build
	substreams run -e $(ENDPOINT_SOL) substreams.yaml map_market_instructions -s 253550034 -t 253550036


.PHONY: map_market_instructions_reg_currency
map_market_instructions_reg_currency: build
	substreams run -e $(ENDPOINT_SOL) substreams.yaml map_market_instructions -s 142400236 -t 142400238

.PHONY: map_market_instructions_failing
map_market_instructions_failing: build
	substreams run -e $(ENDPOINT_SOL) substreams.yaml map_market_instructions -s 222058999 -t 222059001


PHONY: map_market_instructions_wrong
map_market_instructions_wrong: build
	substreams run -e $(ENDPOINT_SOL) substreams.yaml map_market_instructions -s 222056038 -t 222056040

PHONY: map_market_instructions_wrong2
map_market_instructions_wrong2: build
	substreams run -e $(ENDPOINT_SOL) substreams.yaml map_market_instructions -s 194961470 -t 194961472


.PHONY: map_sa_trades_test_001
map_sa_trades_test_001: build
	substreams run -e mainnet.sol.streamingfast.io:443 substreams.yaml map_market_instructions --start-block 223481954 --stop-block 223482000




## TESTS
.PHONY: test_process_exchange
test_process_exchange: build
	substreams run -e $(ENDPOINT_SOL) substreams.yaml map_market_instructions -s 223187376 -t 223187378

.PHONY: test_process_initalize_buy
test_process_initalize_buy: build
	substreams run -e $(ENDPOINT_SOL) substreams.yaml map_market_instructions -s 223187474 -t 223187476

.PHONY: test_process_initalize_sell
test_process_initalize_sell: build
	substreams run -e $(ENDPOINT_SOL) substreams.yaml map_market_instructions -s 222797376 -t 222797378


.PHONY: map_test_process_initalize_cancel
map_test_process_initalize_cancel: build
	substreams run -e $(ENDPOINT_SOL) substreams.yaml map_market_instructions -s 222797306 -t 222797308


.PHONY: db_out
db_out: build
	substreams run -e $(ENDPOINT_SOL) substreams.yaml db_out -s 222797230 -t 222798333



.PHONY: gui
gui: build
	substreams gui -e $(ENDPOINT_SOL) substreams.yaml map_market_instructions -s 222797230 -t 222798333

.PHONY: protogen
protogen:
	substreams protogen ./substreams.yaml --exclude-paths="google,sf/substreams/rpc,sf/substreams/v1"

.PHONY: package
package:
	substreams pack ./substreams.yaml
