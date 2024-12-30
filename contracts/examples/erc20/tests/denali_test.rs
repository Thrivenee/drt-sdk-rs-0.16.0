use numbat_wasm::*;
use numbat_wasm_debug::*;

fn contract_map() -> ContractMap<TxContext> {
	let mut contract_map = ContractMap::new();
	contract_map.register_contract(
		"file:../output/erc20.wasm",
		Box::new(|context| Box::new(erc20::contract_obj(context))),
	);
	contract_map
}

#[test]
fn allowance_callercaller() {
	parse_execute_denali("denali/allowance_CallerCaller.scen.json", &contract_map());
}

#[test]
fn allowance_callerother() {
	parse_execute_denali("denali/allowance_CallerOther.scen.json", &contract_map());
}

#[test]
fn allowance_othercaller() {
	parse_execute_denali("denali/allowance_OtherCaller.scen.json", &contract_map());
}

#[test]
fn allowance_othereqother() {
	parse_execute_denali("denali/allowance_OtherEqOther.scen.json", &contract_map());
}

#[test]
fn allowance_otherneqother() {
	parse_execute_denali("denali/allowance_OtherNEqOther.scen.json", &contract_map());
}

#[test]
fn approve_caller_positive() {
	parse_execute_denali("denali/approve_Caller-Positive.scen.json", &contract_map());
}

#[test]
fn approve_caller_zero() {
	parse_execute_denali("denali/approve_Caller-Zero.scen.json", &contract_map());
}

#[test]
fn approve_other_positive() {
	parse_execute_denali("denali/approve_Other-Positive.scen.json", &contract_map());
}

#[test]
fn approve_other_zero() {
	parse_execute_denali("denali/approve_Other-Zero.scen.json", &contract_map());
}

#[test]
fn approve_switchcaller() {
	parse_execute_denali("denali/approve_SwitchCaller.scen.json", &contract_map());
}

#[test]
fn balanceof_caller() {
	parse_execute_denali("denali/balanceOf_Caller.scen.json", &contract_map());
}

#[test]
fn balanceof_noncaller() {
	parse_execute_denali("denali/balanceOf_NonCaller.scen.json", &contract_map());
}

#[test]
fn not_payable() {
	parse_execute_denali("denali/not_payable.scen.json", &contract_map());
}

#[test]
fn totalsupply_positive() {
	parse_execute_denali("denali/totalSupply_Positive.scen.json", &contract_map());
}

#[test]
fn totalsupply_zero() {
	parse_execute_denali("denali/totalSupply_Zero.scen.json", &contract_map());
}

#[test]
fn transfer_caller_allowanceirrelevant() {
	parse_execute_denali(
		"denali/transfer_Caller-AllowanceIrrelevant.scen.json",
		&contract_map(),
	);
}

#[test]
fn transfer_caller_entirebalance() {
	parse_execute_denali(
		"denali/transfer_Caller-EntireBalance.scen.json",
		&contract_map(),
	);
}

#[test]
fn transfer_caller_morethanbalance() {
	parse_execute_denali(
		"denali/transfer_Caller-MoreThanBalance.scen.json",
		&contract_map(),
	);
}

#[test]
fn transfer_caller_nooverflow() {
	parse_execute_denali(
		"denali/transfer_Caller-NoOverflow.scen.json",
		&contract_map(),
	);
}

#[test]
fn transfer_caller_positive() {
	parse_execute_denali("denali/transfer_Caller-Positive.scen.json", &contract_map());
}

#[test]
fn transfer_caller_stillnooverflow() {
	parse_execute_denali(
		"denali/transfer_Caller-StillNoOverflow.scen.json",
		&contract_map(),
	);
}

#[test]
fn transfer_caller_zero() {
	parse_execute_denali("denali/transfer_Caller-Zero.scen.json", &contract_map());
}

#[test]
fn transferfrom_alldistinct_balanceeqallowance() {
	parse_execute_denali(
		"denali/transferFrom_AllDistinct-BalanceEqAllowance.scen.json",
		&contract_map(),
	);
}

#[test]
fn transferfrom_alldistinct_balanceneqallowance() {
	parse_execute_denali(
		"denali/transferFrom_AllDistinct-BalanceNEqAllowance.scen.json",
		&contract_map(),
	);
}

#[test]
fn transferfrom_alldistinct_entireallowancemorethanbalance() {
	parse_execute_denali(
		"denali/transferFrom_AllDistinct-EntireAllowanceMoreThanBalance.scen.json",
		&contract_map(),
	);
}

#[test]
fn transferfrom_alldistinct_entirebalanceeqallowance() {
	parse_execute_denali(
		"denali/transferFrom_AllDistinct-EntireBalanceEqAllowance.scen.json",
		&contract_map(),
	);
}

#[test]
fn transferfrom_alldistinct_entirebalancemorethanallowance() {
	parse_execute_denali(
		"denali/transferFrom_AllDistinct-EntireBalanceMoreThanAllowance.scen.json",
		&contract_map(),
	);
}

#[test]
fn transferfrom_alldistinct_morethanallowancelessthanbalance() {
	parse_execute_denali(
		"denali/transferFrom_AllDistinct-MoreThanAllowanceLessThanBalance.scen.json",
		&contract_map(),
	);
}

#[test]
fn transferfrom_alldistinct_morethanbalancelessthanallowance() {
	parse_execute_denali(
		"denali/transferFrom_AllDistinct-MoreThanBalanceLessThanAllowance.scen.json",
		&contract_map(),
	);
}

#[test]
fn transferfrom_alldistinct_nooverflow() {
	parse_execute_denali(
		"denali/transferFrom_AllDistinct-NoOverflow.scen.json",
		&contract_map(),
	);
}

#[test]
fn transferfrom_alldistinct_stillnooverflow() {
	parse_execute_denali(
		"denali/transferFrom_AllDistinct-StillNoOverflow.scen.json",
		&contract_map(),
	);
}

#[test]
fn transferfrom_allequal_allowancerelevant() {
	parse_execute_denali(
		"denali/transferFrom_AllEqual-AllowanceRelevant.scen.json",
		&contract_map(),
	);
}

#[test]
fn transferfrom_allequal_entirebalance() {
	parse_execute_denali(
		"denali/transferFrom_AllEqual-EntireBalance.scen.json",
		&contract_map(),
	);
}

#[test]
fn transferfrom_callereqfrom_allowancerelevant() {
	parse_execute_denali(
		"denali/transferFrom_CallerEqFrom-AllowanceRelevant.scen.json",
		&contract_map(),
	);
}

#[test]
fn transferfrom_callereqfrom_entirebalance() {
	parse_execute_denali(
		"denali/transferFrom_CallerEqFrom-EntireBalance.scen.json",
		&contract_map(),
	);
}

#[test]
fn transferfrom_callereqfrom_morethanbalance() {
	parse_execute_denali(
		"denali/transferFrom_CallerEqFrom-MoreThanBalance.scen.json",
		&contract_map(),
	);
}

#[test]
fn transferfrom_callereqto_balanceneqallowance() {
	parse_execute_denali(
		"denali/transferFrom_CallerEqTo-BalanceNEqAllowance.scen.json",
		&contract_map(),
	);
}

#[test]
fn transferfrom_callereqto_morethanallowancelessthanbalance() {
	parse_execute_denali(
		"denali/transferFrom_CallerEqTo-MoreThanAllowanceLessThanBalance.scen.json",
		&contract_map(),
	);
}

#[test]
fn transferfrom_callereqto_morethanbalancelessthanallowance() {
	parse_execute_denali(
		"denali/transferFrom_CallerEqTo-MoreThanBalanceLessThanAllowance.scen.json",
		&contract_map(),
	);
}

#[test]
fn transferfrom_exploratory_multipletransferssucceed() {
	parse_execute_denali(
		"denali/transferFrom_Exploratory-MultipleTransfersSucceed.scen.json",
		&contract_map(),
	);
}

#[test]
fn transferfrom_exploratory_multipletransfersthrow() {
	parse_execute_denali(
		"denali/transferFrom_Exploratory-MultipleTransfersThrow.scen.json",
		&contract_map(),
	);
}

#[test]
fn transferfrom_fromeqto_balanceeqallowance() {
	parse_execute_denali(
		"denali/transferFrom_FromEqTo-BalanceEqAllowance.scen.json",
		&contract_map(),
	);
}

#[test]
fn transferfrom_fromeqto_balanceneqallowance() {
	parse_execute_denali(
		"denali/transferFrom_FromEqTo-BalanceNEqAllowance.scen.json",
		&contract_map(),
	);
}

#[test]
fn transferfrom_fromeqto_entireallowancemorethanbalance() {
	parse_execute_denali(
		"denali/transferFrom_FromEqTo-EntireAllowanceMoreThanBalance.scen.json",
		&contract_map(),
	);
}

#[test]
fn transferfrom_fromeqto_entirebalanceeqallowance() {
	parse_execute_denali(
		"denali/transferFrom_FromEqTo-EntireBalanceEqAllowance.scen.json",
		&contract_map(),
	);
}

#[test]
fn transferfrom_fromeqto_entirebalancemorethanallowance() {
	parse_execute_denali(
		"denali/transferFrom_FromEqTo-EntireBalanceMoreThanAllowance.scen.json",
		&contract_map(),
	);
}

#[test]
fn transferfrom_fromeqto_morethanallowancelessthanbalance() {
	parse_execute_denali(
		"denali/transferFrom_FromEqTo-MoreThanAllowanceLessThanBalance.scen.json",
		&contract_map(),
	);
}

#[test]
fn transferfrom_fromeqto_morethanbalancelessthanallowance() {
	parse_execute_denali(
		"denali/transferFrom_FromEqTo-MoreThanBalanceLessThanAllowance.scen.json",
		&contract_map(),
	);
}

#[test]
fn transferfrom_fromeqto_nooverflow() {
	parse_execute_denali(
		"denali/transferFrom_FromEqTo-NoOverflow.scen.json",
		&contract_map(),
	);
}

#[test]
fn transfer_other_allowanceirrelevant() {
	parse_execute_denali(
		"denali/transfer_Other-AllowanceIrrelevant.scen.json",
		&contract_map(),
	);
}

#[test]
fn transfer_other_entirebalance() {
	parse_execute_denali(
		"denali/transfer_Other-EntireBalance.scen.json",
		&contract_map(),
	);
}

#[test]
fn transfer_other_morethanbalance() {
	parse_execute_denali(
		"denali/transfer_Other-MoreThanBalance.scen.json",
		&contract_map(),
	);
}

#[test]
fn transfer_other_nooverflow() {
	parse_execute_denali(
		"denali/transfer_Other-NoOverflow.scen.json",
		&contract_map(),
	);
}

#[test]
fn transfer_other_positive() {
	parse_execute_denali("denali/transfer_Other-Positive.scen.json", &contract_map());
}

#[test]
fn transfer_other_stillnooverflow() {
	parse_execute_denali(
		"denali/transfer_Other-StillNoOverflow.scen.json",
		&contract_map(),
	);
}

#[test]
fn transfer_other_zero() {
	parse_execute_denali("denali/transfer_Other-Zero.scen.json", &contract_map());
}
