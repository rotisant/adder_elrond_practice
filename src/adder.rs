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
    fn init(&self, initial_value: BigUint) {
        self.sum().set(initial_value);
    }

    /// Add desired amount to the storage variable.
    #[endpoint]
    fn add(&self, value: BigUint) {
        self.sum().update(|sum| *sum += value);
    }

    // #[view]
    // fn temp_sum(&self) -> BigUint {
    //     let biguint_result = self
    //         .contract_proxy(self.blockchain().get_sc_address())
    //         .sum()
    //         .execute_on_dest_context();
    //     return biguint_result;
    // }

    // #[endpoint]
    // fn temp_add(&self, value: BigUint) {
    //     self.contract_proxy(self.blockchain().get_sc_address())
    //         .add(value)
    //         .execute_on_dest_context();
    // }

    #[payable("EGLD")]
    #[endpoint]
    fn temp_delegate(&self, value: BigUint) {
        let sc_address = ManagedAddress::new_from_bytes(&hex!(
            "0000000000000000000100000000000000000000000000000000000006ffffff"
        ));
        self.contract_proxy(sc_address)
            .delegate()
            .with_egld_transfer(value)
            .transfer_execute();
    }

    #[proxy]
    fn contract_proxy(&self, to: ManagedAddress) -> callee_proxy::Proxy<Self::Api>;
}

// mod callee_proxy {
//     elrond_wasm::imports!();

//     #[elrond_wasm::proxy]
//     pub trait CalleeContract {
//         #[view(getSum)]
//         #[storage_mapper("sum")]
//         fn sum(&self) -> SingleValueMapper<BigUint>;

//         #[endpoint]
//         fn add(&self, value: BigUint);
//     }
// }

mod callee_proxy {
    elrond_wasm::imports!();

    #[elrond_wasm::proxy]
    pub trait CalleeContract {
        // #[payable("EGLD")]
        // #[endpoint]
        // fn delegate(&self, #[payment] payment: BigUint);

        #[payable("EGLD")]
        #[endpoint]
        fn delegate(&self);

        // #[payable("EGLD")]
        // #[endpoint(stake)]
        // fn stake_endpoint(&self, #[payment] payment: BigUint);
    }
}
