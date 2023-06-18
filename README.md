Requirements:
1. Rust
2. Foundry https://book.getfoundry.sh/getting-started/installation <br>
3. ```forge install transmissions11/solmate```

Setup (FHE_NODE):
1. Get the forked fhe.rs library at <br>
   ```git clone https://github.com/shankars99/fhe.rs.git```
2. Spin up your own node <br>
   ```cargo run```
3. KEYS <br>
   1. Create you own accounts <br>
   OR
   2. Move your `fhe_private_key` to `/keys`
4. Run the front-end program
   ```npm run dev```

This is a warped privacy token that uses fully homomorphic encryption scheme (based on RING-LWE). <br>

This project contains 2 parts:
1. The Rust crypto engine
2. The frontend javascript
3. REMIX for the batching precompile on moonbase alpha 

The protocol contains 3 main parts:
1. Creating an account and registering yourself with the network, this makes your fhe_public_key public so that others can send you tokens. 
2. Now you can send and receive transactions that you collect to prove you own the amount of tokens that you claim you have
3. Transactions here are quite different from what you'd expect on an EVM network. It's similar to how bitcoin works with a sender and receiver tx, one pointing to the address you want to send the tokens to and the other pointing to your own fhe account.
4. The transactions are all encrypted under the receiver and sender public keys. So now no one can trace or figure out your Tx as they're all encrypted.
5. When you want to withdraw tokens you end up creating a transaction that exposes your fhe_sk for a subset of the tokens. The network verifies it and sends it back.