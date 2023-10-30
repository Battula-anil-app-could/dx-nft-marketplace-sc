dharitri_sc::imports!();

#[dharitri_sc::module]
pub trait CommonUtilFunctions: dharitri_sc_modules::pause::PauseModule {
    fn get_nft_info(&self, nft_type: &TokenIdentifier, nft_nonce: u64) -> DctTokenData<Self::Api> {
        self.blockchain().get_dct_token_data(
            &self.blockchain().get_sc_address(),
            nft_type,
            nft_nonce,
        )
    }
}
