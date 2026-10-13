use thiserror::Error;

#[derive(Debug, Error)]
pub enum BookError {
    #[error("The ID is missing")]
    IDMissing,
    #[error("The seller ID is missing")]
    SellerIDMissing,
    #[error("The buyer ID is missing")]
    BuyerIDMissing,
    #[error("The Price is missing")]
    PriceMissing,
    #[error("The Quantity is missing")]
    QuantityMissing,
    #[error("The Side is missing")]
    SideMissing,
    #[error("The Asks is empty")]
    AsksEmpty,
    #[error("The Bids is empty")]
    BidsEmpty,
}