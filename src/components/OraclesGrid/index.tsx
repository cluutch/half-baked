import { Card, Col, Row  } from "antd";
import React, { useCallback, useState } from "react";
import { useConnection } from "../../contexts/connection";
import { notify } from "../../utils/notifications";
import { PublicKey } from "@solana/web3.js";
import { Button } from "antd";

const { Meta } = Card;

interface Oracle {
  description: string;
}

export const OraclesGrid = () => {
  const connection = useConnection();
  const publicKey = new PublicKey("4pFWkSKheQHCPXQQvZP1sNQyViuXDXvsyw1fXrQECSPn");
  const [oracles, setOracles] = useState<Oracle[]>([]);
  const refresh = useCallback(() => {
    if (!publicKey) {
      return;
    }

  
    connection.getAccountInfo(publicKey).then((metaData) => {
      if (!metaData) return;
      const dat = metaData.data.slice(0, 32);
      console.log("Got dat " + dat);
      
      const apiData = new PublicKey(dat);
      console.log("But is " + apiData);

      connection.getAccountInfo(apiData).then((apiData) => {
        if (!apiData) return;
        const dat = apiData.data.slice(0, 99);

        const description = dat.toString('utf8');
        setOracles([{ description }]);

        notify({
          message: "LOADED THE API DATA " + description,
          type: "success",
        });
      });
    });
  }, [publicKey, connection]);

  // const oracles = [
  //   {
  //     endpointUrlHttps: "api.cluutch.io/v2/daily\?date\=2021-06-05",
  //     lastValue: "$159.00",
  //     timestamp: 1234567,
  //     description: "Price for 1oz of weed",
  //     iconUrlHttps: "cluutch.io/clutch.svg"
  //   },
  //   {
  //     endpointUrlHttps: "datausa.io/api/data?drilldowns=Nation&measures=Population&year=latest",
  //     lastValue: "327167439",
  //     timestamp: 1234567,
  //     description: "Population of USA",
  //     iconUrlHttps: "datausa.io/images/logo_sm.png"
  //   },
  //   {
  //     endpointUrlHttps: "random-data-api.com/api/name/random_name",
  //     lastValue: "Hannelore",
  //     timestamp: 1234567,
  //     description: "Random first name",
  //     iconUrlHttps: "res.cloudinary.com/manoylo/image/upload/v1583284399/manoylome_dhxyc0.png"
  //   },
  //   {
  //     endpointUrlHttps: "random-data-api.com/api/name/random_name",
  //     lastValue: "7236345754",
  //     timestamp: 1234567,
  //     description: "Random positive integer",
  //     iconUrlHttps: "res.cloudinary.com/manoylo/image/upload/v1583284399/manoylome_dhxyc0.png"
  //   },
  //   {
  //     endpointUrlHttps: "www.metaweather.com/api/location/968019",
  //     lastValue: "20210606: 18.53C",
  //     timestamp: 1234567,
  //     description: "Temperature in Brussels",
  //     iconUrlHttps: "www.metaweather.com/static/img/weather/lc.svg"
  //   }
  // ]

  // 
  const cards = oracles.map(oracle => {
    return (
      <Col span={6}>
        <Card
          key={oracle.description}
          hoverable
          cover={<img alt={oracle.description} src={"https://res.cloudinary.com/manoylo/image/upload/v1583284399/manoylome_dhxyc0.png"} />}
        >
          <Meta title={oracle.description} />
          <h1>7236345754</h1>
          <h5>Source: <a href={"https://random-data-api.com/api/name/random_name"}>random-data-api.com/api/name/random_name</a></h5>
          <h6>Last fetched: 1234567</h6>
        </Card>
      </Col>
    )
  })

  // const cards = oracles.map(oracle => {
  //   return (
  //     <Col span={6}>
  //       <Card
  //         key={oracle.description}
  //         hoverable
  //         cover={<img alt={oracle.description} src={"https://" + oracle.iconUrlHttps} />}
  //       >
  //         <Meta title={oracle.description} />
  //         <h1>{oracle.lastValue}</h1>
  //         <h5>Source: <a href={"https://" + oracle.endpointUrlHttps}>{oracle.endpointUrlHttps}</a></h5>
  //         <h6>Last fetched: {oracle.timestamp}</h6>
  //       </Card>
  //     </Col>
  //   )
  // })
  return (
    <>
      <h1>Browse Oracles</h1>
      <Button onClick={refresh}>Refresh</Button>
      <Row align="middle">
          {cards}
      </Row>
      
    </>
  )
};
