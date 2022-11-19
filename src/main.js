import "regenerator-runtime/runtime";
import * as nearAPI from "near-api-js";
import getConfig from "./config";

const nearConfig = getConfig(process.env.NODE_ENV || "development");

// Price for the voting (1 NEAR token)
const votingPrice = "1000000000000000000000000"

async function connect(nearConfig) {
  // Connects to NEAR and provides `near`, `walletAccount` and `contract` objects in `window` scope
  // Initializing connection to the NEAR node.
  window.near = await nearAPI.connect({
    deps: {
      keyStore: new nearAPI.keyStores.BrowserLocalStorageKeyStore()
    },
    ...nearConfig
  });

  // Needed to access wallet login
  window.walletConnection = new nearAPI.WalletConnection(window.near);

  // Initializing our contract APIs by contract name and configuration.
  window.contract = await new nearAPI.Contract(window.walletConnection.account(), nearConfig.contractName, {
    // View methods are read-only – they don't modify the state, but usually return some value
    viewMethods: ['new','get_scoreboard', 'get_list_of_voters', 'get_voting_by_name', 'is_voter_exist'],
    // Change methods can modify the state, but you don't receive the returned value when called
    changeMethods: ['update_scoreboard_with_list'],
    // Sender is the account ID to initialize transactions.
    // getAccountId() will return empty string if user is still unauthorized
    sender: window.walletConnection.getAccountId()
  });
}

function errorHelper(err) {
  // if there's a cryptic error, provide more helpful feedback and instructions here
  // TODO: as soon as we get the error codes propagating back, use those
  if (err.message.includes('Cannot deserialize the contract state')) {
    console.warn('NEAR Warning: the contract/account seems to have state that is not (or no longer) compatible.\n' +
        'This may require deleting and recreating the NEAR account as shown here:\n' +
        'https://stackoverflow.com/a/60767144/711863');
  }
  if (err.message.includes('Cannot deserialize the contract state')) {
    console.warn('NEAR Warning: the contract/account seems to have state that is not (or no longer) compatible.\n' +
        'This may require deleting and recreating the NEAR account as shown here:\n' +
        'https://stackoverflow.com/a/60767144/711863');
  }
  console.error(err);
}

/**
 * Updates the user interface
 */
function updateUI(){
  // if we haven't signed in yet
  if (!window.walletConnection.getAccountId()) {    
        Array.from(document.querySelectorAll('.sign-in')).map(it => it.style = 'display: block;');        
  } else {
    // get account name
    var voter_name = window.walletConnection.getAccountId();
    // check whether account has already voted
    contract.is_voter_exist({voter: voter_name}).then(result => {
      if (result == true) {
        // get account's votes and show them 
        contract.get_voting_by_name({name: window.walletConnection.getAccountId()}).then(result => {
          show_old_voting(result);
        }); 
        document.querySelector('#vote').disabled = true;
        document.querySelectorAll('select').forEach(select => select.disabled = true);
        Array.from(document.querySelectorAll('.after-sign-in .voting')).map(it => it.style = 'display: block;');
        Array.from(document.querySelectorAll('.after-sign-in .results')).map(it => it.style = 'display: block;');

      } else {
        Array.from(document.querySelectorAll('.after-sign-in .voting')).map(it => it.style = 'display: block;');
      }
    });
  }
}

/**
 * Inserts voting results from HashMap to the table
 * @param {Map} scoreboardMap - HashMap with voting results
 */
function insert_values_to_table(scoreboardMap) {
  const sortedScoreboardMap = new Map([...scoreboardMap].sort((a, b) => b[1] - a[1]));
  var table = document.getElementById("scoreboard");
  if (table.rows.length != 1) {
    for(var i = 1;i<table.rows.length;){
      table.deleteRow(i);
    }
  } 
  var i = 1;
  for (const [key, value] of sortedScoreboardMap) {
    var row = table.insertRow(i);
    var cell1 = row.insertCell(0);
    var cell2 = row.insertCell(1);
    cell1.innerHTML = key;
    cell2.innerHTML = value;
    i += 1;
  }
}

/**
 * Checks whether the input array has unique values
 * @param {Array} input_array - array with coutries' names
 * @returns {bool} - boolean value input array has unique values or not
 */
function check_input_array_uniqueness(input_array) {
  if (new Set(input_array).size == 10) {
    return true;
  } else {
    return false;
  }
}

/**
 * Shows the voting made by account before. Set values to the drop-down list objects.
 * @param {Array} input_array - array with coutries' names
 */
function show_old_voting(input_array) {
  var pointsArray = ['12', '10', '8', '7', '6', '5', '4', '3', '2', '1'];
  var pointsArrayLength = pointsArray.length;
  for (var i = 0; i < pointsArrayLength; i++) {
    var country = input_array[i];
    document.getElementById(pointsArray[i]).value = country;
  }
}

// Log in user using NEAR Wallet on "Sign In" button click
document.querySelector('.sign-in .btn').addEventListener('click', () => {
  walletConnection.requestSignIn(nearConfig.contractName, 'Rust Counter Example');
});

// On "View results" button click
document.querySelector('.after-sign-in .results .btn').addEventListener('click', () =>{
 contract.get_scoreboard().then(result => {
    insert_values_to_table(new Map(Object.entries(result)));  
    document.querySelector('#scoreboard').style.display = 'block';
    updateUI();}).catch(err => errorHelper(err));
});

// On "Vote" button click
document.querySelector('.after-sign-in .btn').addEventListener('click', () =>{ 
  var pointsArray = ['12', '10', '8', '7', '6', '5', '4', '3', '2', '1'];
  var pointsArrayLength = pointsArray.length;
  var resultsArray = [];

  for (var i = 0; i < pointsArrayLength; i++) {
    var id_selected = document.getElementById(pointsArray[i]).selectedIndex;
    resultsArray.push(document.getElementById(pointsArray[i])[id_selected].value)
  }
  
  if (check_input_array_uniqueness(resultsArray)) {
    contract.update_scoreboard_with_list({input_list: resultsArray, voter: window.walletConnection.getAccountId()}, "", votingPrice).then(_ => {
      updateUI();
    });
  } else {
    window.confirm("Error! Selected countries should be unique!");
  }
});

document.querySelector('.after-sign-in .voting .sign-out .btn').addEventListener('click', () => {
  walletConnection.signOut();
  // TODO: Move redirect to .signOut() ^^^
  window.location.replace(window.location.origin + window.location.pathname);
});

window.nearInitPromise = connect(nearConfig)
    .then(updateUI)
    .catch(console.error);
