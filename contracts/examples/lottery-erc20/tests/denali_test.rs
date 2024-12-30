use numbat_wasm::*;
use numbat_wasm_debug::*;

fn contract_map() -> ContractMap<TxContext> {
	let mut contract_map = ContractMap::new();
	contract_map.register_contract(
		"file:../output/lottery-erc20.wasm",
		Box::new(|context| Box::new(lottery_erc20::contract_obj(context))),
	);

	contract_map.register_contract(
		"file:../../erc20/output/erc20.wasm",
		Box::new(|context| Box::new(erc20::contract_obj(context))),
	);

	contract_map
}

#[test]
fn lottery_init() {
	parse_execute_denali("denali/lottery-init.scen.json", &contract_map());
}

#[test]
fn buy_all_tickets_different_accounts() {
	parse_execute_denali(
		"denali/buy-all-tickets-different-accounts.scen.json",
		&contract_map(),
	);
}

#[test]
fn buy_more_tickets_than_allowed() {
	parse_execute_denali(
		"denali/buy-more-tickets-than-allowed.scen.json",
		&contract_map(),
	);
}

#[test]
fn buy_ticket_after_deadline() {
	parse_execute_denali(
		"denali/buy-ticket-after-deadline.scen.json",
		&contract_map(),
	);
}

#[test]
fn buy_ticket_after_determined_winner() {
	parse_execute_denali(
		"denali/buy-ticket-after-determined-winner.scen.json",
		&contract_map(),
	);
}

#[test]
fn buy_ticket_after_sold_out() {
	parse_execute_denali(
		"denali/buy-ticket-after-sold-out.scen.json",
		&contract_map(),
	);
}

#[test]
fn buy_ticket_all_options() {
	parse_execute_denali("denali/buy-ticket-all-options.scen.json", &contract_map());
}

#[test]
fn buy_ticket_another_account() {
	parse_execute_denali(
		"denali/buy-ticket-another-account.scen.json",
		&contract_map(),
	);
}

#[test]
fn buy_ticket_not_on_whitelist() {
	parse_execute_denali(
		"denali/buy-ticket-not-on-whitelist.scen.json",
		&contract_map(),
	);
}

#[test]
fn buy_ticket_same_account() {
	parse_execute_denali("denali/buy-ticket-same-account.scen.json", &contract_map());
}

#[test]
fn buy_ticket_second_lottery() {
	parse_execute_denali(
		"denali/buy-ticket-second-lottery.scen.json",
		&contract_map(),
	);
}

#[test]
fn buy_ticket_wrong_fee() {
	parse_execute_denali("denali/buy-ticket-wrong-fee.scen.json", &contract_map());
}

#[test]
fn buy_ticket() {
	parse_execute_denali("denali/buy-ticket.scen.json", &contract_map());
}

#[test]
fn determine_winner_different_ticket_holders_winner_acc1() {
	parse_execute_denali(
		"denali/determine-winner-different-ticket-holders-winner-acc1.scen.json",
		&contract_map(),
	);
}

#[test]
fn determine_winner_early() {
	parse_execute_denali("denali/determine-winner-early.scen.json", &contract_map());
}

#[test]
fn determine_winner_same_ticket_holder() {
	parse_execute_denali(
		"denali/determine-winner-same-ticket-holder.scen.json",
		&contract_map(),
	);
}

// TODO: uncomment after rust-denali supports chaining async calls
// #[test]
// fn determine_winner_split_prize_pool() {
//     parse_execute_denali("denali/determine-winner-split-prize-pool.scen.json", &contract_map());
// }

#[test]
fn start_after_announced_winner() {
	parse_execute_denali(
		"denali/start-after-announced-winner.scen.json",
		&contract_map(),
	);
}

#[test]
fn start_all_options_bigger_whitelist() {
	parse_execute_denali(
		"denali/start-all-options-bigger-whitelist.scen.json",
		&contract_map(),
	);
}

#[test]
fn start_alternative_function_name() {
	parse_execute_denali(
		"denali/start-alternative-function-name.scen.json",
		&contract_map(),
	);
}

#[test]
fn start_fixed_deadline() {
	parse_execute_denali("denali/start-fixed-deadline.scen.json", &contract_map());
}

#[test]
fn start_limited_tickets_and_fixed_deadline_invalid_deadline() {
	parse_execute_denali(
		"denali/start-limited-tickets-and-fixed-deadline-invalid-deadline.scen.json",
		&contract_map(),
	);
}

#[test]
fn start_limited_tickets_and_fixed_deadline_invalid_ticket_price_arg() {
	parse_execute_denali(
		"denali/start-limited-tickets-and-fixed-deadline-invalid-ticket-price-arg.scen.json",
		&contract_map(),
	);
}

#[test]
fn start_limited_tickets_and_fixed_deadline() {
	parse_execute_denali(
		"denali/start-limited-tickets-and-fixed-deadline.scen.json",
		&contract_map(),
	);
}

#[test]
fn start_limited_tickets() {
	parse_execute_denali("denali/start-limited-tickets.scen.json", &contract_map());
}

#[test]
fn start_second_lottery() {
	parse_execute_denali("denali/start-second-lottery.scen.json", &contract_map());
}

#[test]
fn start_with_all_options() {
	parse_execute_denali("denali/start-with-all-options.scen.json", &contract_map());
}

#[test]
fn start_with_no_options() {
	parse_execute_denali("denali/start-with-no-options.scen.json", &contract_map());
}
