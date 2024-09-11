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
        RequestLocked,
        UnauthorizedBuyer,
        OfferAlreadyAccepted,
    }

    pub type Result<T> = core::result::Result<T, MarketplaceError>;

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
        authority: AccountId,
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
        authority: AccountId,
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

    #[derive(Clone, PartialEq)]
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

    #[derive(Clone, PartialEq)]
    #[cfg_attr(
        feature = "std",
        derive(Debug, Eq, ink::storage::traits::StorageLayout)
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
        TIME_TO_LOCK: u64,
        user_ids: Mapping<u64, AccountId>,
    }

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
                TIME_TO_LOCK: 900 * 1000,
                user_ids: Mapping::default(),
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
                authority: caller.clone(),
            };

            self.users.insert(&caller, &new_user);
            self.user_ids.insert(self.user_counter, &caller);
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
            store_name: String,
        ) -> Result<()> {
            let caller = self.env().caller();

            // Fetch user and validate seller status
            let user = self
                .users
                .get(caller)
                .ok_or(MarketplaceError::InvalidUser)?;

            if user.account_type != AccountType::Seller {
                return Err(MarketplaceError::OnlySellersAllowed);
            }

            // Fetch the request and validate its existence
            let mut request = self
                .requests
                .get(request_id)
                .ok_or(MarketplaceError::InvalidRequest)?;

            // Check if the request is locked due to timeout or lifecycle status
            if self.env().block_timestamp()
                > request.updated_at.checked_add(self.TIME_TO_LOCK).unwrap()
                && request.lifecycle == RequestLifecycle::AcceptedByBuyer
            {
                return Err(MarketplaceError::RequestLocked);
            }

            // Increment offer counter and create new offer
            self.offer_counter = self.offer_counter.checked_add(1).unwrap();

            let new_offer = Offer {
                id: self.offer_counter,
                price,
                images: images.clone(),
                request_id,
                store_name: store_name.clone(),
                seller_id: user.id,
                is_accepted: false,
                created_at: self.env().block_timestamp(),
                updated_at: self.env().block_timestamp(),
                authority: caller.clone(),
            };

            // Insert the new offer into storage
            self.offers.insert(self.offer_counter, &new_offer);

            if request.lifecycle == RequestLifecycle::Pending {
                request.lifecycle = RequestLifecycle::AcceptedBySeller;
            }

            // Update the request with the new seller and offer details
            request.seller_ids.push(user.id);
            request.offer_ids.push(self.offer_counter);
            self.requests.insert(request_id, &request);

            // Emit event for offer creation
            self.env().emit_event(OfferCreated {
                offer_id: self.offer_counter,
                seller_address: caller,
                store_name: store_name.clone(),
                price,
                request_id,
                images,
                seller_id: user.id,
                seller_ids: request.seller_ids.clone(),
            });

            Ok(())
        }

        #[ink(message)]
        pub fn accept_offer(&mut self, offer_id: u64) -> Result<()> {
            let caller = self.env().caller();

            // Fetch the offer and validate its existence
            let mut offer = self
                .offers
                .get(offer_id)
                .ok_or(MarketplaceError::InvalidOffer)?;

            let request_id = offer.request_id;

            // Fetch the request and validate its existence
            let mut request = self
                .requests
                .get(request_id)
                .ok_or(MarketplaceError::InvalidRequest)?;

            // Ensure the caller is the authorized buyer for this request
            let buyer = self
                .users
                .get(caller)
                .ok_or(MarketplaceError::InvalidUser)?;

            if buyer.account_type != AccountType::Buyer {
                return Err(MarketplaceError::OnlyBuyersAllowed);
            }

            if request.buyer_id != buyer.id {
                return Err(MarketplaceError::UnauthorizedBuyer);
            }

            // Check if the offer has already been accepted
            if offer.is_accepted {
                return Err(MarketplaceError::OfferAlreadyAccepted);
            }

            // Check if the request is locked due to timeout or lifecycle status
            if self.env().block_timestamp()
                > request.updated_at.checked_add(self.TIME_TO_LOCK).unwrap()
                && request.lifecycle == RequestLifecycle::AcceptedByBuyer
            {
                return Err(MarketplaceError::RequestLocked);
            }

            // Update previous offers for the same request to set `is_accepted` to false
            for offer_id in request.offer_ids.iter() {
                if let Some(mut previous_offer) = self.offers.get(*offer_id) {
                    if previous_offer.is_accepted && previous_offer.request_id == request_id {
                        previous_offer.is_accepted = false;
                        self.offers.insert(*offer_id, &previous_offer);

                        // Emit event for un-accepting the previous offer
                        self.env().emit_event(OfferAccepted {
                            offer_id: previous_offer.id,
                            buyer_address: caller,
                            is_accepted: false,
                        });
                    }
                }
            }

            // Accept the current offer
            offer.is_accepted = true;
            self.offers.insert(offer_id, &offer);
            request.locked_seller_id = offer.seller_id;
            request.sellers_price_quote = offer.price;
            request.lifecycle = RequestLifecycle::AcceptedByBuyer;
            request.updated_at = self.env().block_timestamp();
            self.requests.insert(request_id, &request);

            // Emit events for request and offer acceptance
            self.env().emit_event(RequestAccepted {
                request_id,
                offer_id,
                seller_id: offer.seller_id,
                updated_at: request.updated_at,
                sellers_price_quote: offer.price,
            });

            self.env().emit_event(OfferAccepted {
                offer_id,
                buyer_address: caller,
                is_accepted: true,
            });

            Ok(())
        }

        #[ink(message)]
        pub fn get_user(&self, user_address: AccountId) -> Option<User> {
            self.users.get(user_address)
        }

        #[ink(message)]
        pub fn get_request(&self, request_id: u64) -> Option<Request> {
            self.requests.get(request_id)
        }

        #[ink(message)]
        pub fn get_offer(&self, offer_id: u64) -> Option<Offer> {
            self.offers.get(offer_id)
        }

        #[ink(message)]
        pub fn get_offer_by_request(&self, request_id: u64) -> Vec<Offer> {
            let mut request_offers = Vec::new();
            let request = self.requests.get(request_id).unwrap();
            for offer_id in request.offer_ids.iter() {
                if let Some(offer) = self.offers.get(*offer_id) {
                    request_offers.push(offer.clone());
                }
            }
            request_offers
        }

        #[ink(message)]
        pub fn get_user_requests(&self, user_address: AccountId) -> Vec<Request> {
            let mut user_requests = Vec::new();
            let user = self.users.get(user_address).unwrap();
            for request_id in 0..=self.request_counter {
                if let Some(request) = self.requests.get(request_id) {
                    if request.buyer_id == user.id {
                        user_requests.push(request.clone());
                    }
                }
            }
            user_requests
        }

        #[ink(message)]
        pub fn get_all_requests(&self) -> Vec<Request> {
            let mut all_requests = Vec::new();
            for request_id in 0..=self.request_counter {
                if let Some(request) = self.requests.get(request_id) {
                    all_requests.push(request.clone());
                }
            }
            all_requests
        }

        #[ink(message)]
        pub fn get_user_stores(&self, user_address: AccountId) -> Vec<Store> {
            let mut user_stores = Vec::new();
            let store_ids = self.user_store_ids.get(user_address).unwrap_or_default();
            for store_id in store_ids.iter() {
                if let Some(store) = self.user_stores.get((user_address, *store_id)) {
                    user_stores.push(store.clone());
                }
            }
            user_stores
        }

        #[ink(message)]
        pub fn get_user_by_id(&self, user_id: u64) -> Option<User> {
            if let Some(account_id) = self.user_ids.get(&user_id) {
                self.users.get(&account_id) // Retrieve the user by the AccountId
            } else {
                None
            }
        }

        #[ink(message)]
        pub fn get_seller_offers(&self, seller_address: AccountId) -> Vec<Offer> {
            let mut seller_offers = Vec::new();
            for offer_id in 0..=self.offer_counter {
                if let Some(offer) = self.offers.get(offer_id) {
                    if offer.authority == seller_address {
                        seller_offers.push(offer.clone());
                    }
                }
            }
            seller_offers
        }
    }
}
