const nearAPI = require('near-api-js');

async function connect(nearConfig) {
  // Connects to NEAR and provides `near`, `walletAccount` and `contract` objects in `window` scope
  // Initializing connection to the NEAR node.
  const keyStore = new nearAPI.keyStores.UnencryptedFileSystemKeyStore(
    "/home/gitpod/.near-credentials/"
  );  
  const near = await nearAPI.connect({
    deps: {
      keyStore,
    },
    nodeUrl: "https://rpc.testnet.near.org",
    networkId: "default"
  });
  
  const account = await near.account("dev-1598612906606-3896128");
  
  const functionCallResponse = await account.functionCall(
    "dev-1598612906606-3896128",
    "increment",
    {}
  );
  const result = nearAPI.providers.getTransactionLastResult(
    functionCallResponse
  );
  console.log(result);
}

connect();