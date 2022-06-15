const nearApi = require("near-api-js");

//const keyStore = new nearApi.keyStores.UnencryptedFileSystemKeyStore(
//    "/home/gitpod/.near-credentials/"
//);

const myArgs = process.argv.slice(2);
const accountId = myArgs[0];
const privateKey = myArgs[1];
const contractName = "dev-1598612906606-3896128";

const keyPair = nearApi.utils.KeyPair.fromString(privateKey);
const keyStore = new nearApi.keyStores.InMemoryKeyStore();

keyStore.setKey("default", accountId, keyPair);

async function connect(nearConfig) {
    // Initializing connection to the NEAR node.
    const near = await nearApi.connect({
        deps: {
            keyStore,
        },
        nodeUrl: "https://rpc.testnet.near.org",
        networkId: "default"
    });

    //const account = await near.account("frol4.testnet");
    const account = await near.account(accountId);

    const functionCallResponse = await account.functionCall(
        contractName,
        "increment",
        {}
    );
    const result = nearApi.providers.getTransactionLastResult(
        functionCallResponse
    );
    console.log(result);
}

connect();