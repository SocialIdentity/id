# ID
This smart contract manages the identity across multiple chains.

## Usage

### Admin
* add-chain(chain-id,channel) -- add channel id to use
* remove-chain(chain-id) -- remove channel
* sync-chain(chain-id)

### Privileged
* add-connection(chain-id, identity, wallet)  -- add a identity/wallet pair for a different chain
* add-connection(identity, wallet) -- defaults to current chain
* remove-connection(chain-id,identity,wallet)
* remove-connection(identity,wallet)
