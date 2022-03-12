#![no_std]

elrond_wasm::imports!();
use elrond_wasm::hex_literal::hex;

/// One of the simplest smart contracts possible,
/// it holds a single variable in storage, which anyone can increment.
#[elrond_wasm::derive::contract]
pub trait Adder {
    #[view(getSum)]
    #[storage_mapper("sum")]
    fn sum(&self) -> SingleValueMapper<BigUint>;

    #[init]
    fn init(&self) {
        self.sum().set(BigUint::zero());
    }

    /// Add desired amount to the storage variable.
    #[endpoint]
    fn add(&self, value: BigUint) {
        self.sum().update(|sum| *sum += value);
    }

    // Below Function will transfer all the EGLD balance of contract to the Delegation Contract
    // Make sure your contract has some existing funds
    #[endpoint]
    #[payable("*")]
    fn temp_delegate(&self) {
        let sc_address = ManagedAddress::new_from_bytes(&hex!(
            "0000000000000000000100000000000000000000000000000000000006ffffff"
        ));
        let my_address = self.blockchain().get_sc_address();
        let contract_balance = self.blockchain().get_balance(&my_address);
        self.contract_proxy(sc_address)
            .delegate()
            .with_egld_transfer(contract_balance)
            .with_gas_limit(12000000)
            .transfer_execute();
    }

    #[endpoint]
    fn temp_claim_rewards(&self) {
        let sc_address = ManagedAddress::new_from_bytes(&hex!(
            "0000000000000000000100000000000000000000000000000000000006ffffff"
        ));
        self.contract_proxy(sc_address)
            .claimRewards()
            .with_gas_limit(12000000)
            .transfer_execute();
    }

    #[endpoint]
    fn temp_re_delegate_rewards(&self) {
        let sc_address = ManagedAddress::new_from_bytes(&hex!(
            "0000000000000000000100000000000000000000000000000000000006ffffff"
        ));
        self.contract_proxy(sc_address)
            .reDelegateRewards()
            .with_gas_limit(12000000)
            .transfer_execute();
    }

    #[endpoint]
    fn temp_un_delegate(&self, value: BigUint) {
        let sc_address = ManagedAddress::new_from_bytes(&hex!(
            "0000000000000000000100000000000000000000000000000000000006ffffff"
        ));
        self.contract_proxy(sc_address)
            .unDelegate(value)
            .with_gas_limit(12000000)
            .transfer_execute();
    }

    #[endpoint]
    fn temp_withdraw(&self) {
        let sc_address = ManagedAddress::new_from_bytes(&hex!(
            "0000000000000000000100000000000000000000000000000000000006ffffff"
        ));
        self.contract_proxy(sc_address)
            .withdraw()
            .with_gas_limit(12000000)
            .transfer_execute();
    }

    #[proxy]
    fn contract_proxy(&self, to: ManagedAddress) -> callee_proxy::Proxy<Self::Api>;
}

mod callee_proxy {
    elrond_wasm::imports!();

    #[elrond_wasm::proxy]
    pub trait CalleeContract {
        #[endpoint]
        #[payable("*")]
        fn delegate(&self);

        #[endpoint]
        fn claimRewards(&self);

        #[endpoint]
        fn reDelegateRewards(&self);

        #[endpoint]
        fn unDelegate(&self, value: BigUint);

        #[endpoint]
        fn withdraw(&self);
    }
}
