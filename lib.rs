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
        RequestNotAccepted,
        RequestNotLocked,
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
        location_enabled: bool,
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
    pub struct RequestRemoved {
        #[ink(topic)]
        request_id: u64,
        #[ink(topic)]
        buyer_address: AccountId,
        removed_at: u64,
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
                location_enabled: false,
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
        pub fn toggle_location(&mut self, enable_location: bool) -> Result<()> {
            let caller = self.env().caller();
            let mut user = self
                .users
                .get(caller)
                .ok_or(MarketplaceError::InvalidUser)?;
            user.location_enabled = enable_location;
            self.users.insert(caller, &user);
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
        pub fn delete_request(&mut self, request_id: u64) -> Result<()> {
            let caller = self.env().caller();

            // Fetch the request, or return an error if it doesn't exist
            let request = self
                .requests
                .get(request_id)
                .ok_or(MarketplaceError::InvalidRequest)?;

            // Check if the caller is the buyer who created the request
            let user = self
                .users
                .get(caller)
                .ok_or(MarketplaceError::InvalidUser)?;

            if request.buyer_id != user.id {
                return Err(MarketplaceError::UnauthorizedBuyer);
            }

            if request.lifecycle != RequestLifecycle::Pending {
                return Err(MarketplaceError::RequestLocked);
            }

            // Remove the request from storage
            self.requests.take(request_id);

            // Emit an event for the removed request
            self.env().emit_event(RequestRemoved {
                request_id,
                buyer_address: caller,
                removed_at: self.env().block_timestamp(),
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
        pub fn mark_request_as_completed(&mut self, request_id: u64) -> Result<()> {
            let caller = self.env().caller();

            let mut request = self
                .requests
                .get(request_id)
                .ok_or(MarketplaceError::InvalidRequest)?;

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

            if request.lifecycle != RequestLifecycle::AcceptedByBuyer {
                return Err(MarketplaceError::RequestNotAccepted);
            }

            if request.updated_at.checked_add(self.TIME_TO_LOCK).unwrap()
                > self.env().block_timestamp()
            {
                return Err(MarketplaceError::RequestNotLocked);
            }

            request.lifecycle = RequestLifecycle::Completed;
            request.updated_at = self.env().block_timestamp();
            self.requests.insert(request_id, &request);

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

    #[cfg(test)]
    mod tests {
        use super::*;
        use ink::env::DefaultEnvironment;

        fn set_buyer_env() {
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            ink::env::test::set_caller::<DefaultEnvironment>(accounts.alice);
            ink::env::test::set_callee::<DefaultEnvironment>(accounts.charlie);
        }

        fn set_seller_env() {
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            ink::env::test::set_caller::<DefaultEnvironment>(accounts.bob);
            ink::env::test::set_callee::<DefaultEnvironment>(accounts.charlie);
        }

        #[test]
        fn test_contract_initialization() {
            set_buyer_env();
            let contract = Marketplace::new();
            assert_eq!(contract.user_counter, 0);
            assert_eq!(contract.store_counter, 0);
            assert_eq!(contract.request_counter, 0);
            assert_eq!(contract.offer_counter, 0);
            assert_eq!(contract.TIME_TO_LOCK, 900 * 1000);
        }

        #[test]
        fn test_create_user() {
            set_buyer_env();
            let mut contract = Marketplace::new();

            let username = "Alice".to_string();
            let phone = "1234567890".to_string();
            let latitude = 12345;
            let longitude = 54321;
            let account_type = AccountType::Buyer;

            let result = contract.create_user(
                username.clone(),
                phone.clone(),
                latitude,
                longitude,
                account_type.clone(),
            );
            assert!(result.is_ok());

            let caller = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>().alice;
            let user = contract.users.get(caller).unwrap();

            assert_eq!(user.username, username);
            assert_eq!(user.phone, phone);
            assert_eq!(user.location.latitude, latitude);
            assert_eq!(user.location.longitude, longitude);
            assert_eq!(user.account_type, account_type);
        }

        #[test]
        fn test_update_user() {
            set_buyer_env();
            let mut contract = Marketplace::new();

            let username = "Alice".to_string();
            let phone = "1234567890".to_string();
            let latitude = 12345;
            let longitude = 54321;
            let account_type = AccountType::Buyer;

            contract
                .create_user(
                    username.clone(),
                    phone.clone(),
                    latitude,
                    longitude,
                    account_type.clone(),
                )
                .unwrap();

            // Update the user
            let new_username = "AliceUpdated".to_string();
            let new_phone = "0987654321".to_string();
            let new_latitude = 67890;
            let new_longitude = 98765;
            let new_account_type = AccountType::Seller;

            let result = contract.update_user(
                new_username.clone(),
                new_phone.clone(),
                new_latitude,
                new_longitude,
                new_account_type.clone(),
            );
            assert!(result.is_ok());

            let caller = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>().alice;
            let user = contract.users.get(caller).unwrap();

            assert_eq!(user.username, new_username);
            assert_eq!(user.phone, new_phone);
            assert_eq!(user.location.latitude, new_latitude);
            assert_eq!(user.location.longitude, new_longitude);
            assert_eq!(user.account_type, new_account_type);
        }

        #[test]
        fn test_create_store() {
            set_buyer_env();
            let mut contract = Marketplace::new();

            // Create a seller
            let username = "Alice".to_string();
            let phone = "1234567890".to_string();
            let latitude = 12345;
            let longitude = 54321;
            let account_type = AccountType::Seller;
            contract
                .create_user(
                    username.clone(),
                    phone.clone(),
                    latitude,
                    longitude,
                    account_type,
                )
                .unwrap();

            // Create a store
            let store_name = "My Store".to_string();
            let store_description = "Best Store".to_string();
            let result = contract.create_store(
                store_name.clone(),
                store_description.clone(),
                phone.clone(),
                latitude,
                longitude,
            );
            assert!(result.is_ok());

            // Check store creation
            let caller = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>().alice;
            let stores = contract.get_user_stores(caller);
            assert_eq!(stores.len(), 1);
            assert_eq!(stores[0].name, store_name);
            assert_eq!(stores[0].description, store_description);
        }

        #[test]
        fn test_create_request() {
            set_buyer_env();
            let mut contract = Marketplace::new();

            // Create a buyer
            let username = "Bob".to_string();
            let phone = "0987654321".to_string();
            let latitude = 98765;
            let longitude = 56789;
            let account_type = AccountType::Buyer;
            contract
                .create_user(
                    username.clone(),
                    phone.clone(),
                    latitude,
                    longitude,
                    account_type,
                )
                .unwrap();

            // Create a request
            let request_name = "Request 1".to_string();
            let request_description = "Need this item".to_string();
            let images = vec!["image1".to_string(), "image2".to_string()];
            let result = contract.create_request(
                request_name.clone(),
                request_description.clone(),
                images.clone(),
                latitude,
                longitude,
            );
            assert!(result.is_ok());

            // Check request creation
            let requests = contract.get_all_requests();
            assert_eq!(requests.len(), 1);
            assert_eq!(requests[0].name, request_name);
            assert_eq!(requests[0].description, request_description);
            assert_eq!(requests[0].images, images);
        }

        #[test]
        fn test_create_offer() {
            set_buyer_env();
            let mut contract = Marketplace::new();

            // Create a buyer and a request
            let buyer_name = "Alice".to_string();
            let buyer_phone = "0987654321".to_string();
            let latitude = 98765;
            let longitude = 56789;
            let buyer_account_type = AccountType::Buyer;
            contract
                .create_user(
                    buyer_name.clone(),
                    buyer_phone.clone(),
                    latitude,
                    longitude,
                    buyer_account_type,
                )
                .unwrap();

            let request_name = "Request 1".to_string();
            let request_description = "Need this item".to_string();
            let images = vec!["image1".to_string()];
            contract
                .create_request(
                    request_name.clone(),
                    request_description.clone(),
                    images.clone(),
                    latitude,
                    longitude,
                )
                .unwrap();

            set_seller_env();

            // Create a seller and a store
            let seller_name = "Bob".to_string();
            let seller_phone = "1234567890".to_string();
            let seller_account_type = AccountType::Seller;
            contract
                .create_user(
                    seller_name.clone(),
                    seller_phone.clone(),
                    latitude,
                    longitude,
                    seller_account_type,
                )
                .unwrap();

            let store_name = "My Store".to_string();
            let store_description = "Best Store".to_string();
            contract
                .create_store(
                    store_name.clone(),
                    store_description,
                    seller_phone.clone(),
                    latitude,
                    longitude,
                )
                .unwrap();

            // Create an offer
            let request_id = 1;
            let offer_price = 100;
            let offer_images = vec!["offer_image1".to_string()];

            contract
                .create_offer(
                    request_id,
                    offer_price,
                    offer_images.clone(),
                    store_name.clone(),
                )
                .unwrap();

            // Check offer creation
            let offers = contract.get_offer_by_request(request_id);
            assert_eq!(offers.len(), 1);
            assert_eq!(offers[0].price, offer_price);
            assert_eq!(offers[0].images, offer_images);
            assert_eq!(offers[0].store_name, store_name);
        }

        #[test]
        fn test_accept_offer() {
            set_buyer_env();
            let mut contract = Marketplace::new();

            // Create a buyer and a request
            let buyer_name = "Bob".to_string();
            let buyer_phone = "0987654321".to_string();
            let latitude = 98765;
            let longitude = 56789;
            let buyer_account_type = AccountType::Buyer;
            contract
                .create_user(
                    buyer_name.clone(),
                    buyer_phone.clone(),
                    latitude,
                    longitude,
                    buyer_account_type,
                )
                .unwrap();

            let request_name = "Request 1".to_string();
            let request_description = "Need this item".to_string();
            let images = vec!["image1".to_string()];
            contract
                .create_request(
                    request_name.clone(),
                    request_description.clone(),
                    images.clone(),
                    latitude,
                    longitude,
                )
                .unwrap();

            set_seller_env();

            // Create a seller and a store
            let seller_name = "Alice".to_string();
            let seller_phone = "1234567890".to_string();
            let seller_account_type = AccountType::Seller;
            contract
                .create_user(
                    seller_name.clone(),
                    seller_phone.clone(),
                    latitude,
                    longitude,
                    seller_account_type,
                )
                .unwrap();

            let store_name = "My Store".to_string();
            let store_description = "Best Store".to_string();
            contract
                .create_store(
                    store_name.clone(),
                    store_description,
                    seller_phone.clone(),
                    latitude,
                    longitude,
                )
                .unwrap();

            // Create an offer
            let request_id = 1;
            let offer_price = 100;
            let offer_images = vec!["offer_image1".to_string()];
            contract
                .create_offer(
                    request_id,
                    offer_price,
                    offer_images.clone(),
                    store_name.clone(),
                )
                .unwrap();

            set_buyer_env();

            // Accept the offer
            let offer_id = 1;
            let result = contract.accept_offer(offer_id);
            assert!(result.is_ok());

            // // Check if the offer was accepted
            let accepted_offer = contract.get_offer(offer_id).unwrap();
            assert_eq!(accepted_offer.is_accepted, true);

            // Check the request lifecycle
            let request = contract.get_request(request_id).unwrap();
            assert_eq!(request.lifecycle, RequestLifecycle::AcceptedByBuyer);
            assert_eq!(request.locked_seller_id, accepted_offer.seller_id);
        }

        #[test]
        fn test_mark_request_as_completed() {
            set_buyer_env();
            let mut contract = Marketplace::new();

            // Create a buyer and a request
            let buyer_name = "Bob".to_string();
            let buyer_phone = "0987654321".to_string();
            let latitude = 98765;
            let longitude = 56789;
            let buyer_account_type = AccountType::Buyer;
            contract
                .create_user(
                    buyer_name.clone(),
                    buyer_phone.clone(),
                    latitude,
                    longitude,
                    buyer_account_type,
                )
                .unwrap();

            let request_name = "Request 1".to_string();
            let request_description = "Need this item".to_string();
            let images = vec!["image1".to_string()];
            contract
                .create_request(
                    request_name.clone(),
                    request_description.clone(),
                    images.clone(),
                    latitude,
                    longitude,
                )
                .unwrap();

            set_seller_env();

            // Create a seller and a store
            let seller_name = "Alice".to_string();
            let seller_phone = "1234567890".to_string();
            let seller_account_type = AccountType::Seller;
            contract
                .create_user(
                    seller_name.clone(),
                    seller_phone.clone(),
                    latitude,
                    longitude,
                    seller_account_type,
                )
                .unwrap();

            let store_name = "My Store".to_string();
            let store_description = "Best Store".to_string();
            contract
                .create_store(
                    store_name.clone(),
                    store_description,
                    seller_phone.clone(),
                    latitude,
                    longitude,
                )
                .unwrap();

            // Create an offer
            let request_id = 1;
            let offer_price = 100;
            let offer_images = vec!["offer_image1".to_string()];
            contract
                .create_offer(
                    request_id,
                    offer_price,
                    offer_images.clone(),
                    store_name.clone(),
                )
                .unwrap();

            set_buyer_env();

            // // Accept the offer
            let offer_id = 1;
            contract.accept_offer(offer_id).unwrap();

            // // Mark the request as completed
            // let result = contract.mark_request_as_completed(request_id).unwrap();
            // assert!(result.is_ok());

            // // Check the request lifecycle
            // let request = contract.get_request(request_id).unwrap();
            // assert_eq!(request.lifecycle, RequestLifecycle::Completed);
        }

        #[test]
        // remove request
        fn test_remove_request() {
            set_buyer_env();
            let mut contract = Marketplace::new();

            // Create a buyer and a request
            let buyer_name = "Bob".to_string();
            let buyer_phone = "0987654321".to_string();
            let latitude = 98765;
            let longitude = 56789;
            let buyer_account_type = AccountType::Buyer;
            contract
                .create_user(
                    buyer_name.clone(),
                    buyer_phone.clone(),
                    latitude,
                    longitude,
                    buyer_account_type,
                )
                .unwrap();

            let request_name = "Request 1".to_string();
            let request_description = "Need this item".to_string();
            let images = vec!["image1".to_string()];
            contract
                .create_request(
                    request_name.clone(),
                    request_description.clone(),
                    images.clone(),
                    latitude,
                    longitude,
                )
                .unwrap();

            // Remove the request
            let request_id = 1;
            let result = contract.delete_request(request_id);
            assert!(result.is_ok());

            // Check if the request was removed
            let request = contract.get_request(request_id);
            assert_eq!(request, None);
        }

        // #[test]
        // pub fn toggle_location() {
        //     set_buyer_env();
        //     let mut contract = Marketplace::new();
    }
}
