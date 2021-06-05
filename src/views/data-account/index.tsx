import React, { useCallback } from "react";
import { useConnection, sendTransaction } from "../../contexts/connection";
import { useWallet } from "../../contexts/wallet";
import { LAMPORTS_PER_SOL, TransactionInstruction, Account, PublicKey } from "@solana/web3.js";
import { notify } from "../../utils/notifications";
import { ConnectButton } from "./../../components/ConnectButton";
import { LABELS } from "../../constants";
import { HALF_BAKED_ID } from "../../utils/ids";

export const DataAccountView = () => {
  const connection = useConnection();
  const { wallet, publicKey } = useWallet();

  const createDataAccount = useCallback(() => {
    if (!publicKey) {
      return;
    }

    if (!wallet) {
      return;
    }

    const apiTransaction = new TransactionInstruction({
      data: Buffer.from([28,62,0,0]),
      keys: [{
        isSigner: false,
        isWritable: true,
        pubkey: new PublicKey("9gcJMPhzWDUAUbGBxW6xvsqaApsWsJAAPzg4mafQdrXe")
      }],
      programId: new PublicKey(HALF_BAKED_ID)
    });
    const instructions: TransactionInstruction[] = [apiTransaction];
    // connection.sendTransaction()
    // const signers: Account[] = [];
    sendTransaction(connection, wallet, instructions).then(() => {
      notify({
        message: "Sent the damn transaction",
        type: "success",
      });
    });
    // connection.requestAirdrop(publicKey, 2 * LAMPORTS_PER_SOL).then(() => {
    //   notify({
    //     message: LABELS.ACCOUNT_FUNDED,
    //     type: "success",
    //   });
    // });
    console.log("FINISHED MAKING THE REQUEST");
    
  }, [publicKey, connection]);

  return (
    <div className="flexColumn" style={{ flex: 1 }}>
      <div>
        <div className="deposit-input-title" style={{ margin: 10 }}>
          Going to write data (15900) to Half Baked
        </div>
        <ConnectButton type="primary" onClick={createDataAccount}>
          WRITE TO API
        </ConnectButton>
      </div>
    </div>
  );
};
