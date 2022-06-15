const nearApi = require("near-api-js");

const keyStore = new nearApi.keyStores.UnencryptedFileSystemKeyStore(
    "/home/gitpod/.near-credentials/"
);

async function connect(nearConfig) {
    // Initializing connection to the NEAR node.
    const near = await nearApi.connect({
        deps: {
            keyStore,
        },
        nodeUrl: "https://rpc.testnet.near.org",
        networkId: "default"
    });

    const account = await near.account("dev-1598612906606-3896128");    

    const functionCallResponse = await account.functionCall(
        'dev-1598612906606-3896128',
        "increment",
        {}
    );
    const result = nearApi.providers.getTransactionLastResult(
        functionCallResponse
    );
    console.log(result);
}

connect();