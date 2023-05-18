import * as web3 from "@solana/web3.js";

import { getCounter, increase } from "./functions";
import { Counter } from "./types";

const connection = new web3.Connection("http://127.0.0.1:8899");

async function main() {
  // write your code here
  await increase(connection);
  console.log(await getCounter(connection))

  await increase(connection);
  console.log(await getCounter(connection))

  let test = new web3.PublicKey("5U4ZvgtA3rskTi9S5cXxdF4DomNeFYUV3j8wWbqxzZPo")
}

main()
  .then(() => process.exit(0))
  .catch((err) => {
    console.error(err);
    process.exit(1);
  });
