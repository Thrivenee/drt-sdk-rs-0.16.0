use numbat_wasm::*;
use numbat_wasm_debug::*;

fn contract_map() -> ContractMap<TxContext> {
	let mut contract_map = ContractMap::new();
	contract_map.register_contract(
		"file:../output/use-module.wasm",
		Box::new(|context| Box::new(use_module::contract_obj(context))),
	);
	contract_map
}

#[test]
fn use_module_features() {
	parse_execute_denali("denali/use_module_features.scen.json", &contract_map());
}

#[test]
fn use_module_pause() {
	parse_execute_denali("denali/use_module_pause.scen.json", &contract_map());
}
