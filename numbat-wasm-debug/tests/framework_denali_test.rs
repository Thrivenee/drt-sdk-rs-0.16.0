use numbat_wasm_debug::*;

// These tests don't really test any contract, but the testing framework itslef.

fn contract_map() -> ContractMap<TxContext> {
	ContractMap::new()
}

/// Checks that externalSteps work fine.
#[test]
fn external_steps() {
	parse_execute_denali(
		"tests/denali/external_steps/external_steps.scen.json",
		&contract_map(),
	);
}

#[test]
fn transfer() {
	parse_execute_denali("tests/denali/transfer.scen.json", &contract_map());
}

#[test]
fn validator_reward() {
	parse_execute_denali("tests/denali/validatorReward.scen.json", &contract_map());
}
