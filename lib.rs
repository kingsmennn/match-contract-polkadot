#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod marketplace {
    use ink::prelude::string::String;
    use ink::prelude::vec::Vec;
    use ink::storage::Mapping;

    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[derive(Debug, PartialEq, Eq)]
    pub enum MarketplaceError {
        UserAlreadyExists,
        InvalidUser,
        OnlySellersAllowed,
        OnlyBuyersAllowed,
        InvalidRequest,
        InvalidOffer,
        InvalidRequestOfferCombination,
    }

    pub type Result<T> = core::result::Result<T, MarketplaceError>;

    #[derive(Default)]
    #[ink(storage)]
    pub struct Marketplace {
        users: Mapping<AccountId, User>,
        requests: Mapping<u64, Request>,
        offers: Mapping<u64, Offer>,
        user_store_ids: Mapping<AccountId, Vec<u64>>,
        user_stores: Mapping<(AccountId, u64), Store>,
        user_counter: u64,
        store_counter: u64,
        request_counter: u64,
        offer_counter: u64,
    }

    #[derive(Clone)]
    #[cfg_attr(
        feature = "std",
        derive(Debug, PartialEq, Eq, ink::storage::traits::StorageLayout)
    )]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    pub struct Location {
        latitude: i64,
        longitude: i64,
    }

    #[derive(Clone)]
    #[cfg_attr(
        feature = "std",
        derive(Debug, PartialEq, Eq, ink::storage::traits::StorageLayout)
    )]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    pub struct Store {
        id: u64,
        name: String,
        description: String,
        phone: String,
        location: Location,
    }

    #[derive(Clone)]
    #[cfg_attr(
        feature = "std",
        derive(Debug, PartialEq, Eq, ink::storage::traits::StorageLayout)
    )]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    pub struct User {
        id: u64,
        username: String,
        phone: String,
        location: Location,
        created_at: u64,
        updated_at: u64,
        account_type: AccountType,
    }

    #[derive(Clone)]
    #[cfg_attr(
        feature = "std",
        derive(Debug, PartialEq, Eq, ink::storage::traits::StorageLayout)
    )]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    pub struct Request {
        id: u64,
        name: String,
        buyer_id: u64,
        sellers_price_quote: i64,
        seller_ids: Vec<u64>,
        offer_ids: Vec<u64>,
        locked_seller_id: u64,
        description: String,
        images: Vec<String>,
        created_at: u64,
        lifecycle: RequestLifecycle,
        location: Location,
        updated_at: u64,
    }

    #[derive(Clone)]
    #[cfg_attr(
        feature = "std",
        derive(Debug, PartialEq, Eq, ink::storage::traits::StorageLayout)
    )]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    pub struct Offer {
        id: u64,
        price: i64,
        images: Vec<String>,
        request_id: u64,
        store_name: String,
        seller_id: u64,
        is_accepted: bool,
        created_at: u64,
        updated_at: u64,
    }

    #[ink(event)]
    pub struct UserCreated {
        #[ink(topic)]
        user_address: AccountId,
        user_id: u64,
        username: String,
        account_type: u8,
    }

    #[ink(event)]
    pub struct UserUpdated {
        #[ink(topic)]
        user_address: AccountId,
        user_id: u64,
        username: String,
        account_type: u8,
    }

    #[ink(event)]
    pub struct StoreCreated {
        #[ink(topic)]
        seller_address: AccountId,
        store_id: u64,
        store_name: String,
        latitude: i64,
        longitude: i64,
    }

    #[ink(event)]
    pub struct OfferAccepted {
        #[ink(topic)]
        offer_id: u64,
        #[ink(topic)]
        buyer_address: AccountId,
        is_accepted: bool,
    }

    #[ink(event)]
    pub struct RequestCreated {
        #[ink(topic)]
        request_id: u64,
        #[ink(topic)]
        buyer_address: AccountId,
        request_name: String,
        latitude: i64,
        longitude: i64,
        images: Vec<String>,
        lifecycle: u8,
        description: String,
        buyer_id: u64,
        seller_ids: Vec<u64>,
        sellers_price_quote: i64,
        locked_seller_id: u64,
        created_at: u64,
        updated_at: u64,
    }

    #[ink(event)]
    pub struct OfferCreated {
        #[ink(topic)]
        offer_id: u64,
        #[ink(topic)]
        seller_address: AccountId,
        store_name: String,
        price: i64,
        request_id: u64,
        images: Vec<String>,
        seller_id: u64,
        seller_ids: Vec<u64>,
    }

    #[ink(event)]
    pub struct RequestAccepted {
        #[ink(topic)]
        request_id: u64,
        #[ink(topic)]
        offer_id: u64,
        #[ink(topic)]
        seller_id: u64,
        updated_at: u64,
        sellers_price_quote: i64,
    }

    #[ink(event)]
    pub struct OfferRemoved {
        #[ink(topic)]
        offer_id: u64,
        #[ink(topic)]
        seller_address: AccountId,
    }

    #[derive(Clone)]
    #[derive(PartialEq)]
    #[cfg_attr(
        feature = "std",
        derive(Debug, Eq, ink::storage::traits::StorageLayout)
    )]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    pub enum AccountType {
        Buyer,
        Seller,
    }

    impl Default for AccountType {
        fn default() -> Self {
            AccountType::Buyer
        }
    }

    #[derive(Clone)]
    #[cfg_attr(
        feature = "std",
        derive(Debug, PartialEq, Eq, ink::storage::traits::StorageLayout)
    )]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    pub enum RequestLifecycle {
        Pending,
        AcceptedBySeller,
        AcceptedByBuyer,
        RequestLocked,
        Completed,
    }

    impl Default for RequestLifecycle {
        fn default() -> Self {
            RequestLifecycle::Pending
        }
    }

    #[ink(event)]
    pub struct RequestLifecycleChanged {
        #[ink(topic)]
        request_id: u64,
        new_lifecycle: u8,
    }

    #[ink(impl)]
    impl Marketplace {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                users: Mapping::default(),
                requests: Mapping::default(),
                offers: Mapping::default(),
                user_store_ids: Mapping::default(),
                user_stores: Mapping::default(),
                user_counter: 0,
                store_counter: 0,
                request_counter: 0,
                offer_counter: 0,
            }
        }

        #[ink(message)]
        pub fn create_user(
            &mut self,
            username: String,
            phone: String,
            latitude: i64,
            longitude: i64,
            account_type: AccountType,
        ) -> Result<()> {
            let caller = self.env().caller();
            let user = self.users.get(caller);
            if user.is_some() {
                return Err(MarketplaceError::UserAlreadyExists);
            }
            self.user_counter = self.user_counter.checked_add(1).unwrap();
            let new_user = User {
                id: self.user_counter,
                username: username.clone(),
                phone,
                location: Location {
                    latitude,
                    longitude,
                },
                created_at: self.env().block_timestamp(),
                updated_at: self.env().block_timestamp(),
                account_type: account_type.clone(),
            };

            self.users.insert(caller, &new_user);
            self.env().emit_event(UserCreated {
                user_address: caller,
                user_id: self.user_counter,
                username,
                account_type: match account_type {
                    AccountType::Buyer => 0,
                    AccountType::Seller => 1,
                },
            });
            Ok(())
        }

        #[ink(message)]
        pub fn update_user(
            &mut self,
            username: String,
            phone: String,
            latitude: i64,
            longitude: i64,
            account_type: AccountType,
        ) -> Result<()> {
            let caller = self.env().caller();
            let mut user = self
                .users
                .get(caller)
                .ok_or(MarketplaceError::InvalidUser)?;

            user.username = username.clone();
            user.phone = phone;
            user.location = Location {
                latitude,
                longitude,
            };
            user.updated_at = self.env().block_timestamp();
            user.account_type = account_type.clone();

            self.users.insert(caller, &user);

            self.env().emit_event(UserUpdated {
                user_address: caller,
                user_id: user.id,
                username,
                account_type: match account_type {
                    AccountType::Buyer => 0,
                    AccountType::Seller => 1,
                },
            });
            Ok(())
        }

        #[ink(message)]
        pub fn create_store(
            &mut self,
            name: String,
            description: String,
            phone: String,
            latitude: i64,
            longitude: i64,
        ) -> Result<()> {
            let caller = self.env().caller();
            let user = self
                .users
                .get(caller)
                .ok_or(MarketplaceError::InvalidUser)?;

            if user.account_type != AccountType::Seller {
                return Err(MarketplaceError::OnlySellersAllowed);
            }

            self.store_counter = self.store_counter.checked_add(1).unwrap();
            let new_store = Store {
                id: self.store_counter,
                name: name.clone(),
                description,
                phone,
                location: Location {
                    latitude,
                    longitude,
                },
            };

            self.user_stores
                .insert((caller, self.store_counter), &new_store);
            let mut store_ids = self.user_store_ids.get(caller).unwrap_or_default();
            store_ids.push(self.store_counter);
            self.user_store_ids.insert(caller, &store_ids);

            self.env().emit_event(StoreCreated {
                seller_address: caller,
                store_id: self.store_counter,
                store_name: name,
                latitude,
                longitude,
            });
            Ok(())
        }

        #[ink(message)]
        pub fn create_request(
            &mut self,
            name: String,
            description: String,
            images: Vec<String>,
            latitude: i64,
            longitude: i64,
        ) -> Result<()> {
            let caller = self.env().caller();
            let user = self
                .users
                .get(caller)
                .ok_or(MarketplaceError::InvalidUser)?;

            if user.account_type != AccountType::Buyer {
                return Err(MarketplaceError::OnlyBuyersAllowed);
            }

            self.request_counter = self.request_counter.checked_add(1).unwrap();
            let new_request = Request {
                id: self.request_counter,
                name: name.clone(),
                buyer_id: user.id,
                sellers_price_quote: 0,
                seller_ids: Vec::new(),
                offer_ids: Vec::new(),
                locked_seller_id: 0,
                description: description.clone(),
                images: images.clone(),
                created_at: self.env().block_timestamp(),
                lifecycle: RequestLifecycle::Pending,
                location: Location {
                    latitude,
                    longitude,
                },
                updated_at: self.env().block_timestamp(),
            };

            self.requests.insert(self.request_counter, &new_request);
            self.env().emit_event(RequestCreated {
                request_id: self.request_counter,
                buyer_address: caller,
                request_name: name,
                latitude,
                longitude,
                images,
                lifecycle: 0,
                description,
                buyer_id: user.id,
                seller_ids: Vec::new(),
                sellers_price_quote: 0,
                locked_seller_id: 0,
                created_at: self.env().block_timestamp(),
                updated_at: self.env().block_timestamp(),
            });
            Ok(())
        }

        #[ink(message)]
        pub fn create_offer(
            &mut self,
            request_id: u64,
            price: i64,
            images: Vec<String>,
        ) -> Result<()> {
            let caller = self.env().caller();
            let user = self
                .users
                .get(caller)
                .ok_or(MarketplaceError::InvalidUser)?;

            if user.account_type != AccountType::Seller {
                return Err(MarketplaceError::OnlySellersAllowed);
            }

            let mut request = self
                .requests
                .get(request_id)
                .ok_or(MarketplaceError::InvalidRequest)?;

            self.offer_counter = self.offer_counter.checked_add(1).unwrap();
            let new_offer = Offer {
                id: self.offer_counter,
                price,
                images: images.clone(),
                request_id,
                store_name: String::from("Sample Store"),
                seller_id: user.id,
                is_accepted: false,
                created_at: self.env().block_timestamp(),
                updated_at: self.env().block_timestamp(),
            };

            self.offers.insert(self.offer_counter, &new_offer);

            request.seller_ids.push(user.id);
            request.offer_ids.push(self.offer_counter);
            request.updated_at = self.env().block_timestamp();
            self.requests.insert(request_id, &request);

            self.env().emit_event(OfferCreated {
                offer_id: self.offer_counter,
                seller_address: caller,
                store_name: String::from("Sample Store"),
                price,
                request_id,
                images,
                seller_id: user.id,
                seller_ids: request.seller_ids.clone(),
            });
            Ok(())
        }

        #[ink(message)]
        pub fn accept_offer(&mut self, request_id: u64, offer_id: u64) -> Result<()> {
            let caller = self.env().caller();
            let mut request = self
                .requests
                .get(request_id)
                .ok_or(MarketplaceError::InvalidRequest)?;
            let mut offer = self
                .offers
                .get(offer_id)
                .ok_or(MarketplaceError::InvalidOffer)?;

            if offer.request_id != request_id {
                return Err(MarketplaceError::InvalidRequestOfferCombination);
            }

            offer.is_accepted = true;
            self.offers.insert(offer_id, &offer);
            request.locked_seller_id = offer.seller_id;
            request.lifecycle = RequestLifecycle::AcceptedByBuyer;
            request.updated_at = self.env().block_timestamp();
            self.requests.insert(request_id, &request);

            self.env().emit_event(OfferAccepted {
                offer_id,
                buyer_address: caller,
                is_accepted: true,
            });
            Ok(())
        }
    }
}
