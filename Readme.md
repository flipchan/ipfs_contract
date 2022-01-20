# A minimal file uploader that uploads a ipfs to and then compiles an ethereum smart contract with the file identifer from ipfs(cid)


## Platform:
Built and tested on debian 10

## Requirements:  
local instance of ipfs(run `sh make install`)
solc(run `sh make install_solc`)
Rust

## Build:  
```shell
$ make install   
$ make build  

```

## Run:   
You can either just run the binary from the target folder or use:
```shell
$ cargo run --release myfile.txt
    Finished release [optimized] target(s) in 0.07s                  
     Running `target/release/ipfs_contract`                          
Welcome to the IPFS file uploader and cid smart contract generator   
usage: target/release/ipfs_contract file_to_upload                   
``` 
Or target/release/ipfs_contract:
```shell   
./target/release/ipfs_contract /tmp/test
Welcome to the IPFS file uploader and cid smart contract generator
Connecting to ipfs on localhost
Uploaded file to ipfs with Cid: "QmeomffUNfmQy76CQGy9NdmqEnnHU9soCexBnGU3ezPHVH"
Generated solidity contract: QmeomffUNfmQy76CQGy9NdmqEnnHU9soCexBnGU3ezPHVH.sol
Compiled the smart contract
You can now deploy the compiled contract and get the cid to your file on ipfs by quering the getfile function

```

