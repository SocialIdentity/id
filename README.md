# ID
This smart contract manages the identity across multiple chains.

## Usage

### Admin
* add-chain(chain-id,channel) -- add channel id to use
* remove-chain(chain-id) -- remove channel
* sync-chain(chain-id)

### Privileged
* add-identity(chain-id, identity, wallet)  -- add a identity/wallet pair for a different chain
* add-identity(identity, wallet) -- defaults to current chain
* remove-identity(chain-id,identity,wallet)
* remove-identity(identity,wallet)
