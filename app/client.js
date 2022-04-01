/* client.js. This client does the following:
Takes two parameters from the command line:
    Program - the program ID of our deployed program
    Feed - the chainlink data feed account that we want to obtain price data from
Uses the passed in program ID parameter, and the generated IDL file for the program, creates a connection to the deployed program
Generates a new keypair to be used for storing the price data
Calls the ‘execute’ instruction on the deployed program, passing in the account to store the price data in in, the user account paying for the transaction fees, the Chainlink data feed account address of the data feed we are obtaining price data from, the Chainlink Price Feeds program on Devnet, and the system program ID. We also sign the transaction with the account storing the price data
Prints out the program log messages obtained once the transaction is confirmed
Obtains the latest price data stored in the consumer account that was passed into the program, and prints the value to the console
*/

// Parse arguments
// --program - [Required] The account address for your deployed program.
// --feed - The account address for the Chainlink data feed to retrieve
const args = require('minimist')(process.argv.slice(2));

// Initialize Anchor and provider
const anchor = require("@project-serum/anchor");
const provider = anchor.Provider.env();
// Configure the cluster.
anchor.setProvider(provider);

const CHAINLINK_PROGRAM_ID = "CaH12fwNTKJAG8PxEvo9R96Zc2j8qNHZaFj8ZW49yZNT";
const DIVISOR = 100000000;

// Data feed account address
// Default is SOL / USD
const default_feed = "EdWr4ww1Dq82vPe8GFjjcVPo2Qno3Nhn6baCgM3dCy28";
const CHAINLINK_FEED = args['feed'] || default_feed;

async function main() {
 // Read the generated IDL.
 const idl = JSON.parse(
   require("fs").readFileSync("../target/idl/solana_chainlink.json", "utf8")
 );

 // Address of the deployed program.
 const programId = new anchor.web3.PublicKey(args['program']);

 // Generate the program client from IDL.
 const program = new anchor.Program(idl, programId);

 //create an account to store the price data
 const priceFeedAccount = anchor.web3.Keypair.generate();

 console.log('priceFeedAccount public key: ' + priceFeedAccount.publicKey);
 console.log('user public key: ' + provider.wallet.publicKey);

 // Execute the RPC.
 let tx = await program.rpc.execute({
   accounts: {
     decimal: priceFeedAccount.publicKey,
     user: provider.wallet.publicKey,
     chainlinkFeed: CHAINLINK_FEED,
     chainlinkProgram: CHAINLINK_PROGRAM_ID,
     systemProgram: anchor.web3.SystemProgram.programId
   },
   options: { commitment: "confirmed" }, //start with the most recent block that's confirmed
   signers: [priceFeedAccount],
 });

 console.log("Fetching transaction logs...");
 let t = await provider.connection.getConfirmedTransaction(tx, "confirmed");
 console.log(t.meta.logMessages);

 // Fetch the account details of the account containing the price data
 const latestPrice = await program.account.decimal.fetch(priceFeedAccount.publicKey);
 console.log('Price Is: ' + latestPrice.value / DIVISOR)
}

console.log("Running client...");
main().then(() => console.log("Success"));