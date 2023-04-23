import * as web3 from "@solana/web3.js";
import { Buffer } from "buffer";
import { programId, SIGNER } from "./constants";
import * as utils from "./utils";
import { Counter } from "./types";

export const increase = async (
  connection: web3.Connection,
  signer: web3.Keypair = SIGNER
) => {
  let instructionNumber = 0;

  let dataBuffer = Buffer.from("");

  dataBuffer = utils.packUInt8(dataBuffer, instructionNumber);

  let [counterAddress, _counterBump] =
    web3.PublicKey.findProgramAddressSync(
      [Buffer.from("counter")],
      programId
    );

  const instruction = new web3.TransactionInstruction({
    programId,
    keys: [
      { pubkey: counterAddress, isSigner: false, isWritable: false },
      { pubkey: signer.publicKey, isSigner: true, isWritable: false },
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

/******* GETTERS ********/

export const getCounter = async (
  connection: web3.Connection,
  signer: web3.Keypair = SIGNER
) => {
  let [counterAddress, _counterBump] =
    web3.PublicKey.findProgramAddressSync(
      [Buffer.from("counter")],
      programId
    );

  console.log("COUNTER ADDRESS: ", counterAddress.toString());

  let counterInfo = await connection.getAccountInfo(
    counterAddress,
    "processed"
  );

  
  
  let data = counterInfo ? counterInfo.data : null;
  if (!data) {
    throw new Error("No data retrieved");
  }

  console.log("COUNTER Data: ", data);

  console.log("Number: ", data.readUint32LE())

  let counterStruct = Counter.decode(data);
  return counterStruct;
};
