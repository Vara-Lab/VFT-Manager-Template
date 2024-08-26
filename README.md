# VFT Manager

Small template (Mini DEXs) that manages the vft contract.
It controls the swap between Varas and the tokens specified on VFT contract.

## Instructions to use both contracts:

1. Compile both contracts.
2. Upload the extended-vft contract to the IDEA, you have to put the necesary data for your token.
3. Upload the mini_dexs contract, you can put an initial value for the contract (or you can put that info in each call to the contract): 
    - vft_contract_id: Some or None, it is the contract if of the vft contract.
    - min_tokens_to_add: Min tokens to the contract in the vft contract.
    - tokens_per_vara: cost of tokens for a Vara (will also be used to change tokens to a Vara).
3. In the vft contract you need to grant admin, burn and minter role to the mini_dexs contract.
4. With that, you can swap Varas and your tokens!

## Try ut on gitpod!

<p align="center">
  <a href="https://gitpod.io/#https://github.com/Vara-Lab/VFT-Manager-Template" target="_blank">
    <img src="https://gitpod.io/button/open-in-gitpod.svg" width="240" alt="Gitpod">
  </a>
</p>
