import React, { useCallback, useState } from "react";
import { useConnection, sendTransaction } from "../../contexts/connection";
import { useWallet } from "../../contexts/wallet";
import { LAMPORTS_PER_SOL, TransactionInstruction, Account, PublicKey } from "@solana/web3.js";
import { notify } from "../../utils/notifications";
import { ConnectButton } from "./../../components/ConnectButton";
import { LABELS } from "../../constants";
import { HALF_BAKED_ID } from "../../utils/ids";
import { Form, Input, Button, Checkbox, Col, Row } from 'antd';
// import { useForm } from "react-hook-form";

interface Oracle {
  description: string;
  endpointUrlHttps: string;
  jsonValPath: string;
  iconUrlHttps: string;
  value: string;
}


export const NewOracleView = () => {
  // const [oracle, setOracle] = useState<Oracle>({
  //   description: "",
  //   endpointUrlHttps: "",
  //   jsonValPath: "",
  //   iconUrlHttps: "",
  //   value: ""
  // });

  const connection = useConnection();
  const { wallet, publicKey } = useWallet();
  // const { register, handleSubmit, watch, formState: { errors } } = useForm();
  const createOracle = useCallback(() => {
    // if (!publicKey) {
    //   return;
    // }

    // if (!wallet) {
    //   return;
    // }

    // const apiTransaction = new TransactionInstruction({
    //   data: Buffer.from([28,62,0,0]),
    //   keys: [{
    //     isSigner: false,
    //     isWritable: true,
    //     pubkey: new PublicKey("9gcJMPhzWDUAUbGBxW6xvsqaApsWsJAAPzg4mafQdrXe")
    //   }],
    //   programId: new PublicKey(HALF_BAKED_ID)
    // });
    // const instructions: TransactionInstruction[] = [apiTransaction];
    // // connection.sendTransaction()
    // // const signers: Account[] = [];
    // sendTransaction(connection, wallet, instructions).then(() => {
    //   notify({
    //     message: "Sent the damn transaction",
    //     type: "success",
    //   });
    // });
    // connection.requestAirdrop(publicKey, 2 * LAMPORTS_PER_SOL).then(() => {
    //   notify({
    //     message: LABELS.ACCOUNT_FUNDED,
    //     type: "success",
    //   });
    // });
    console.log("FINISHED MAKING THE REQUEST");
    
  }, [publicKey, connection]);

  const onFinish = (oracle: Oracle) => {
    // setOracle({
    //   description: "",
    //   endpointUrlHttps: "",
    //   jsonValPath: "",
    //   iconUrlHttps: "",
    //   jsonVal: "TODO"
    // });
    console.log('Success:', oracle);
  };

  const onFinishFailed = (errorInfo: any) => {
    console.log('Failed:', errorInfo);
  };

  return (
    <Row align="middle">
      <Col span={24}>
        <Form
          name="basic"
          initialValues={{ remember: true }}
          onFinish={onFinish}
          onFinishFailed={onFinishFailed}
        >
          <Form.Item
            label="API Endpoint"
            name="endpointUrlHttps"
            rules={[{ required: true, message: 'Please input the API endpoints URL. Include everything after https://, including query params.' }]}
          >
            <Input />
          </Form.Item>

          <Form.Item
            label="Short Description"
            name="description"
            rules={[{ required: true, message: 'Please input a short description' }]}
          >
            <Input />
          </Form.Item>

          <Form.Item
            label="JSON Value path"
            name="jsonValPath"
            rules={[{ required: true, message: 'Please input how to lookup the API response in the JSON payload returned' }]}
          >
            <Input />
          </Form.Item>

          <Form.Item
            label="Icon URL"
            name="iconUrlHttps"
            rules={[{ required: true, message: 'Please input a public image to use for the oracle.  Include everything after https://, including query params.' }]}
          >
            <Input />
          </Form.Item>

          <Form.Item
            label="Value"
            name="value"
            rules={[{ required: true, message: 'Please input value to store as the most recent API response' }]}
          >
            <Input />
          </Form.Item>

          <Form.Item>
            <ConnectButton type="primary" htmlType="submit" onClick={createOracle}>
              WRITE TO API
            </ConnectButton>
          </Form.Item>
        </Form>
        
      </Col>
    </Row>
  );
};
