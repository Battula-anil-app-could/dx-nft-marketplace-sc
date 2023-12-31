// Code generated by the dharitri-sc multi-contract system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                           23
// Async Callback (empty):               1
// Total number of exported functions:  25

#![no_std]
#![feature(alloc_error_handler, lang_items)]

dharitri_sc_wasm_adapter::allocator!();
dharitri_sc_wasm_adapter::panic_handler!();

dharitri_sc_wasm_adapter::endpoints! {
    dct_nft_marketplace
    (
        setCutPercentage
        auctionToken
        endAuction
        withdraw
        getFullAuctionData
        getLastValidAuctionId
        getMarketplaceCutPercentage
        sendOffer
        withdrawOffer
        acceptOffer
        withdrawAuctionAndAcceptOffer
        getFullOfferData
        getLastValidOfferId
        bid
        buySft
        claimTokens
        getClaimableAmount
        addTokensToWhitelist
        removeTokensFromWhitelist
        getWhitelistedTokens
        pause
        unpause
        isPaused
    )
}

dharitri_sc_wasm_adapter::empty_callback! {}
