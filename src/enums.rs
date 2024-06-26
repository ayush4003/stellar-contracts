use soroban_sdk::contracttype;

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum DataKey {
    Admin,
    InitializedLedger,
    NativeToken,
    AuctionList,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum Status {
    Ok,
    Fail,
    AlreadyInitialized,
    NotInitialized,
    TryAgain,
    NotFound,
    Found,
    Saved,
    Bumped,
    Upgraded,
    Duplicate,
    InsufficientFund,
    Underage,
    MissingOwnership,
    FullStomach,
    OnAuction,
    BidIsClosed,
    BidIsOpen,
    CannotBidLower,
    NameAlreadyExist,
}

#[contracttype]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum CowBreed {
    Jersey = 1,
    Limousin = 2,
    Hallikar = 3,
    Hereford = 4,
    Holstein = 5,
    Simmental = 6,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum CowGender {
    Male = 1,
    Female = 2,
}
