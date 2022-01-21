use anyhow::Error;
/// Simple ethereum solc wrapper
use std::fs::File;
use std::io::Write;
use std::process::Command;

/// Generate ethereum/solidity smart contract
pub fn generate_contract(cid: &str) -> Result<String, Error> {
    let mut long_string = String::new(); // create a string buffer and write the contract, because format! doesn't allow us to format "{"
    long_string.push_str(
        r#"
pragma solidity ^0.8.11;
  
contract Inbox {
    string public filename; 
    string ipfs_file = ""#,
    );
    long_string.push_str(format!("{filename}", filename = cid).as_str()); // cid of ipfs file
    long_string.push_str(
        r#"";
    constructor(string memory myfile) {
        filename = myfile; //set the filename
    }

    function getfile() public view returns(string memory){
      return ipfs_file; 
   }
         
    function Changefilename(string memory newName) public {
        filename = newName;
    }
}

    "#,
    );
    // write to file
    let mut file: File =
        File::create(format!("{filename}.sol", filename = cid)).expect("Could not create file");
    file.write_all(long_string.as_bytes())?;
    println!("Generated solidity contract: {}.sol", cid);
    let _compileme = compile_solc(&format!("{}.sol", cid).as_str());
    Ok(long_string)
}

/// Make the source code tiny and use the installed version of solc instead of a library: solc --optimize --bin test.sol
pub fn compile_solc(contract_file: &str) -> Result<bool, Error> {
    let shelloutput = Command::new("solc")
        .arg("--optimize") // compiler optimize the code
        .arg("--bin")
        .arg(&contract_file)
        .output()
        .unwrap_or_else(|e| panic!("To run solc, here is the error: {}", e)); // use solc to compile the contract without any thirdparty wrapper

    let compiled_solidity = String::from_utf8_lossy(&shelloutput.stdout);
    let mut file: File = File::create(format!("{filename}.bin", filename = contract_file))
        .expect("Could not create file");
    file.write_all(compiled_solidity.as_bytes())?;

    println!("Compiled the smart contract");
    println!("You can now deploy the compiled contract and get the cid to your file on ipfs by quering the getfile function");

    Ok(true)
}
