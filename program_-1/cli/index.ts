import * as web3 from '@solana/web3.js';

import { callProgram0 } from './functions';
import { unpackUInt8 } from './utils';
const connection = new web3.Connection('http://127.0.0.1:8899');

async function main() {
  // write your code here
  const program0Id = new web3.PublicKey("8c85ctRTjrdfHsBM8xLeY9LeAEZj4wCBUF7SaaFouWTe")

  await callProgram0(
    connection,
    program0Id,
    66
  );

  // Print Adder.number in console
  let adderInfo = await connection.getAccountInfo(
    new web3.PublicKey('7CvGzVBGnpuBWsAt8h5AEN8SB9HCzunYrs4FstKVM2Wf'),
    'processed'
  );

  let data = adderInfo ? adderInfo.data : null;
  if (!data) {
    throw new Error('No data retrieved');
  }
 
  const [num, _] = unpackUInt8(data);

  console.log('Counter number is: ', num);

}

main()
  .then(() => process.exit(0))
  .catch((err) => {
    console.error(err);
    process.exit(1);
  });
