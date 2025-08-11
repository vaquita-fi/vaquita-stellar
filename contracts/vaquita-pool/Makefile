include ../../.env

default: build

all: test

test: build
	cargo test

build:
	stellar contract build
	@ls -l ../../target/wasm32v1-none/release/*.wasm


deploy: build
	stellar contract deploy --source $(SOURCE_ACCOUNT) --network testnet --wasm ../../target/wasm32v1-none/release/vaquita_pool.wasm --alias vaquita_pool

initialize: deploy
	stellar contract invoke --source $(SOURCE_ACCOUNT) --network testnet --id vaquita_pool \
	-- \
	initialize \
	--owner $(USER_ADDRESS) \
	--token CAQCFVLOBK5GIULPNZRGATJJMIZL5BSP7X5YJVMGCPTUEPFM4AVSRCJU \
	--pool_address CCLBPEYS3XFK65MYYXSBMOGKUI4ODN5S7SUZBGD7NALUQF64QILLX5B5 \
	--lock_periods "[604800]"

deposit:
	stellar contract invoke --id vaquita_pool --source $(SOURCE_ACCOUNT) --network testnet \
	-- \
	deposit \
	--caller $(USER_ADDRESS) \
	--deposit_id $(DEPOSIT_ID) \
	--amount 10000000 \
	--period 604800

withdraw:
	stellar contract invoke --id vaquita_pool --source $(SOURCE_ACCOUNT) --network testnet \
	-- \
	withdraw \
	--caller $(USER_ADDRESS) \
	--deposit_id $(DEPOSIT_ID)

fmt:
	cargo fmt --all

clean:
	cargo clean