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


.PHONY: map_market_instructions_from_142385016
map_market_instructions_from_142385016: build
	substreams run -e $(ENDPOINT_SOL) substreams.yaml map_market_instructions -s 142385016 -t 143385016


.PHONY: map_market_instructions_reg_currency
map_market_instructions_reg_currency: build
	substreams run -e $(ENDPOINT_SOL) substreams.yaml map_market_instructions -s 142400236 -t 142400238



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
