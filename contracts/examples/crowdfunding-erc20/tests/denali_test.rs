use numbat_wasm::*;
use numbat_wasm_debug::*;

fn contract_map() -> ContractMap<TxContext> {
	let mut contract_map = ContractMap::new();

	contract_map.register_contract(
		"file:../output/crowdfunding-erc20.wasm",
		Box::new(|context| Box::new(crowdfunding_erc20::contract_obj(context))),
	);

	contract_map.register_contract(
		"file:../../erc20/output/erc20.wasm",
		Box::new(|context| Box::new(erc20::contract_obj(context))),
	);

	contract_map
}

#[test]
fn deploy_erc20_and_crowdfunding() {
	parse_execute_denali(
		"denali/deploy_erc20_and_crowdfunding.scen.json",
		&contract_map(),
	);
}

#[test]
fn fund_without_allowance() {
	parse_execute_denali("denali/fund_without_allowance.scen.json", &contract_map());
}

#[test]
fn fund_with_insufficient_allowance() {
	parse_execute_denali(
		"denali/fund_with_insufficient_allowance.scen.json",
		&contract_map(),
	);
}

#[test]
fn fund_with_sufficient_allowance() {
	parse_execute_denali(
		"denali/fund_with_sufficient_allowance.scen.json",
		&contract_map(),
	);
}
