# VFT Manager

VFT-Manager template that manages the vft contract.
It controls the swap between Varas and the tokens specified on VFT contract.

## Instructions to use both contracts:

1. Compile both contracts.
2. Upload the extended-vft contract to the IDEA, you have to put the necesary data for your token.
3. Upload the mini_dexs contract, you can put an initial value for the contract (or you can put that info in each call to the contract): 
    - vft_contract_id: Some or None, it is the contract if of the vft contract.
    - min_tokens_to_add: Min tokens to the contract in the vft contract.
    - tokens_per_vara: cost of tokens for a Vara (will also be used to change tokens to a Vara).
4. In the vft contract you need to grant admin, burn and minter role to the mini_dexs contract.
5. Add balance to VFT-Manager with add_tokens_to_contract function.
6. With that, you can swap Varas and your tokens!


## Step 1: Open Contract on Gitpod

<p align="center">
  <a href="https://gitpod.io/#https://github.com/Vara-Lab/VFT-Manager-Template" target="_blank">
    <img src="https://gitpod.io/button/open-in-gitpod.svg" width="240" alt="Gitpod">
  </a>
</p>

## Step 2: Compile and Deploy the Smart Contract

### Rust: You need to have rust 1.80 or newer to be able to compile your contract:

```bash
rustup install 1.81
rustup default 1.81
rustup target add wasm32-unknown-unknown
```
### Compile the smart contract by running the following command:

```bash
cargo build --release
```

Once the compilation is complete, locate the `*.opt.wasm` file in the `target/wasm32-unknown-unknown/release` directory.


## Step 3: Interact with Your Contract on Vara Network

1. To interact with the Gear IDEA and deploy your contract, you will need to download a wallet extension such as [Polkadot-JS](https://polkadot.js.org/extension/), [Talisman](https://talisman.xyz/), or [Subwallet](https://subwallet.app/) to interact with Substrate-based chains.

<div align="center">
  <img src="https://polkadot.js.org/extension/extension-overview.png" alt="Polkadot-JS Extension">
</div>


## Step 4: Interact with Your Contract on Vara Network

1. Access [Gear IDE](https://idea.gear-tech.io/programs?node=wss%3A%2F%2Frpc.vara.network) using your web browser.
2. Connect your Substrate wallet to Gear IDE.
3. Upload the `*.opt.wasm` and `metadata.txt` files by clicking the "Upload Program" button.

**Vara Standards**: [Standards](https://github.com/gear-foundation/standards.git)  


## Try ut on gitpod!

<p align="center">
  <a href="https://gitpod.io/#https://github.com/Vara-Lab/VFT-Manager-Template" target="_blank">
    <img src="https://gitpod.io/button/open-in-gitpod.svg" width="240" alt="Gitpod">
  </a>
</p>
