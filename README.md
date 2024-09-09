This repository contains a Polkadot smart contract for a decentralized marketplace built with Ink!. The contract allows users to create buyer/seller profiles, create stores and requests, and submit or accept offers.

### Structs and Enums

- **MarketplaceError**: Defines possible errors the contract can encounter, such as user-related issues or request/offer mishandling.
- **Marketplace**: The main contract structure. It contains mappings for users, requests, offers, stores, counters, and a locking mechanism for requests.
- **Location, Store, User, Request, Offer**: Structures that define the data schema for various entities in the marketplace, such as user information, store details, and marketplace transactions.
- **AccountType**: Enum to differentiate between buyers and sellers.
- **RequestLifecycle**: Enum that tracks the state of a request, such as pending, accepted by buyer/seller, locked, and completed.

### Main Functionalities

1. **User Management**:

   - `create_user`: Adds a new user to the contract. It checks if the user already exists and emits a `UserCreated` event upon success.
   - `update_user`: Updates an existing user's information, emitting a `UserUpdated` event.

2. **Store Management**:

   - `create_store`: Allows a user with a seller account to create a store. The store is stored under the user's account, and a `StoreCreated` event is emitted.

3. **Request Management**:

   - `create_request`: Buyers can create a request for goods/services, specifying their location, description, and images. This action emits a `RequestCreated` event.

4. **Offer Management**:
   - `create_offer`: Sellers can create offers for a buyer's request. It checks if the request is locked and if the seller is authorized, then adds the offer to the marketplace and emits an `OfferCreated` event.
   - `accept_offer`: Buyers can accept offers created for their requests. This function ensures only the correct buyer accepts the offer and handles the lifecycle updates for the request.

### Error Handling

The contract handles various error conditions (e.g., `UserAlreadyExists`, `UnauthorizedBuyer`, `RequestLocked`) to ensure marketplace integrity. It also checks conditions such as if an offer has already been accepted or if the user is authorized to perform certain actions.

To run and build the Ink! smart contract in the `Marketplace` module, follow these steps:

### Prerequisites

1. **Rust Setup**:

   - Make sure you have Rust installed. You can install it using:
     ```bash
     curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
     ```

2. **Cargo Contract**:

   - Install `cargo-contract` which is required to build Ink! smart contracts:
     ```bash
     cargo install cargo-contract --force --locked
     ```

3. **Substrate Node**:
   - You'll need a local Substrate-based blockchain or Polkadot development environment like a `Canvas` node or `substrate-node-template` to deploy and interact with the smart contract.

### Build the Marketplace Contract

1. Clone the repository (if it's on GitHub or another repo):

   ```bash
   git clone https://github.com/kingsmennn/match-contract-polkadot.git
   cd match-contract-polkadot
   ```

2. Compile the contract:

   ```bash
   cargo contract build
   ```

   This will generate a `.contract` file in the `target` folder, which includes the compiled WebAssembly (`.wasm`) and metadata files required to deploy the contract.

### Running the Local Development Chain

1. Run a local Substrate development chain:
   - If you have `substrate-contracts-node`, you can start it by running:
     ```bash
     substrate-contracts-node --dev --tmp
     ```

### Deploying the Contract

1. Go to the PolkadotJS Apps portal:

   - Open [PolkadotJS Apps](https://polkadot.js.org/apps/#/contracts).

2. Upload and Deploy the Contract:
   - Navigate to the "Contracts" tab and click on "Deploy Contract."
   - Upload the `.contract` file that was generated during the build process.
   - Set the initial parameters (if any) and deploy the contract.

### Interacting with the Contract

1. Once deployed, you can interact with the contract via the PolkadotJS UI by calling the smart contract's messages such as `create_user`, `create_store`, `create_request`, and `create_offer`.

2. You can also run tests or write scripts to interact with the contract programmatically.

### Testing

- If you want to test the contract logic before deployment, write unit tests inside the contract using `#[ink::test]` and run:
  ```bash
  cargo test
  ```

## License

This project is licensed under the MIT License.
