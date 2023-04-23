import * as web3 from '@solana/web3.js';
import { Accumulator } from './types';
import { programId, SIGNER, programBId } from './constants';

export const callIncreaseAndLogCounterFromProgramB = async (
  connection: web3.Connection,
  programBPubkey: web3.PublicKey,
  num: number,
  signer: web3.Keypair = SIGNER
) => {
  let dataBuffer = Buffer.from('');

  dataBuffer = packUInt32(dataBuffer, num)

  let [accumulatorAddress] = await web3.PublicKey.findProgramAddressSync(
    [Buffer.from('counter')],
    programBPubkey
  );

  // console.log('dataBuffer: ', dataBuffer);

  const instruction = new web3.TransactionInstruction({
    programId,
    keys: [
      { pubkey: programBPubkey, isSigner: false, isWritable: false },
      { pubkey: accumulatorAddress, isSigner: false, isWritable: true },
      { pubkey: signer.publicKey, isSigner: true, isWritable: true },
      {
        pubkey: web3.SystemProgram.programId,
        isSigner: false,
        isWritable: true,
      },
    ],
    data: dataBuffer,
  });

  let txReceipt = await web3.sendAndConfirmTransaction(
    connection,
    new web3.Transaction().add(instruction),
    [signer]
  );
  return txReceipt;
};

export const getAccumulator = async (
  connection: web3.Connection,
  signer: web3.Keypair = SIGNER
) => {
  let [AccumulatorAddress, _AccumulatorBump] =
    web3.PublicKey.findProgramAddressSync(
      [Buffer.from("counter")],
      programBId
    );
  let AccumulatorInfo = await connection.getAccountInfo(
    AccumulatorAddress,
    "processed"
  );
  let data = AccumulatorInfo ? AccumulatorInfo.data : null;
  if (!data) {
    throw new Error("No data retrieved");
  }
  let AccumulatorStruct = Accumulator.decode(data);
  return AccumulatorStruct;
};

export const packUInt32 = (buf: Buffer, data: number): Buffer => {
  let newArrayBuffer = new ArrayBuffer(buf.length + 4);
  let newBuffer = Buffer.from(newArrayBuffer);
  newBuffer.set(buf);
  newBuffer.writeUInt32LE(data, buf.length);
  return newBuffer;
};
