use anyhow::Result;
use ipfs_api_backend_hyper::{IpfsApi, IpfsClient};
use std::fs::File;
/// ipfs cid contract generator, tested and built on debian 10, rustc 1.58.0 (02072b482 2022-01-11) | Goal:
// Take a file as an argument from the command line
// upload file to ipfs
// Store the CID in a smart contract
// Deploy the contract?
use std::{env, process};

mod simple_eth;

/// Display help msg then exit
fn show_help(myname: &str) -> ! {
    println!("usage: {} file_to_upload", myname);
    process::exit(1);
}

// use tokio's async functionality
#[ipfs_contract::main]
async fn main() -> Result<()> {
    println!("Welcome to the IPFS file uploader and cid smart contract generator"); //description
    let args: Vec<String> = env::args().collect(); //parse sys arguments
    if args.len() != 2 {
        // return help if not the correct amount of arguments are given
        show_help(&args[0]);
    }

    println!("Connecting to ipfs on localhost");
    let client = IpfsClient::default(); // connect to ipfs on localhost
    let file = File::open(&args[1]).expect("Could not open file"); // open the file to upload

    let cid: String = client.add(file).await?.hash; // upload the file to ipfs
    println!("Uploaded file to ipfs with Cid: {:?}", cid); // print the cid of the file

    let _contract_hash: String = simple_eth::generate_contract(&cid)?; //keep incase we want to add functionality(then we can define the correct type and so on) to deploy the contract with infura

    Ok(())
}

/// test, echo "test" > /tmp/test

#[cfg(test)]
mod tests {
    use ipfs_api_backend_hyper::{IpfsApi, IpfsClient};
    use std::fs::File;
    async fn setup() -> String {
        let client = IpfsClient::default(); // connect to ipfs on localhost
        let file = File::open(&"/tmp/test").expect("Could not open file"); // open the file to upload
        let wait = client.add(file).await;
        let cid = match wait {
            Ok(cid) => cid.hash,
            Err(e) => panic!("{}", e),
        };
        cid
    }

    #[tokio::test(flavor = "multi_thread")] // thanks tokio for enabling async tests into Rust
    async fn test_ipfs() {
        println!("Running test");
        let cid = setup().await;
        println!("Uploaded file to ipfs with Cid: {:?}", cid);
        assert_eq!(
            cid.as_str(),
            "QmeomffUNfmQy76CQGy9NdmqEnnHU9soCexBnGU3ezPHVH"
        );
        assert!(true);
    }
}
