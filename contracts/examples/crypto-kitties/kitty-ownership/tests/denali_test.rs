use numbat_wasm::*;
use numbat_wasm_debug::*;

fn contract_map() -> ContractMap<TxContext> {
	let mut contract_map = ContractMap::new();

	contract_map.register_contract(
		"file:../../kitty-genetic-alg/output/kitty-genetic-alg.wasm",
		Box::new(|context| Box::new(kitty_genetic_alg::contract_obj(context))),
	);
	contract_map.register_contract(
		"file:../output/kitty-ownership.wasm",
		Box::new(|context| Box::new(kitty_ownership::contract_obj(context))),
	);

	contract_map
}

#[test]
fn init() {
	parse_execute_denali("denali/init.scen.json", &contract_map());
}

#[test]
fn setup_accounts() {
	parse_execute_denali("denali/setup_accounts.scen.json", &contract_map());
}

#[test]
fn query() {
	parse_execute_denali("denali/query.scen.json", &contract_map());
}

#[test]
fn approve_siring() {
	parse_execute_denali("denali/approve_siring.scen.json", &contract_map());
}

#[test]
fn breed_ok() {
	parse_execute_denali("denali/breed_ok.scen.json", &contract_map());
}

#[test]
fn give_birth() {
	parse_execute_denali("denali/give_birth.scen.json", &contract_map());
}
