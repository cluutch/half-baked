import { Card, Col, Row  } from "antd";
import React, { useCallback, useState } from "react";
import { useConnection } from "../../contexts/connection";
import { notify } from "../../utils/notifications";
import { PublicKey } from "@solana/web3.js";
import { Button } from "antd";

const { Meta } = Card;

interface Oracle {
  endpointUrlHttps: string;
  description: string;
  value: string;
  jsonValuePath: string;
  iconUrlHttps: string;
}

export const OraclesGrid = () => {
  
  const connection = useConnection();
  const metaDataPubKey = new PublicKey("52iJqwHvBsUhEaNg3wJJLzNSBzksaVC6kFSV18w215ZJ");
  const [oracles, setOracles] = useState([
    {
      endpointUrlHttps: "api.cluutch.io/v2/daily\?date\=2021-06-05",
      description: "Price for 1oz of weed",
      value: "$159.00",
      jsonValuePath: "price",
      iconUrlHttps: "cluutch.io/clutch.svg"
    },
    {
      endpointUrlHttps: "datausa.io/api/data?drilldowns=Nation&measures=Population&year=latest",
      value: "327167439",
      description: "Population of USA",
      iconUrlHttps: "datausa.io/images/logo_sm.png",
      jsonValuePath: "data.Population"
    },
    {
      endpointUrlHttps: "www.metaweather.com/api/location/968019",
      value: "18.53C",
      description: "Temperature in Brussels",
      iconUrlHttps: "www.metaweather.com/static/img/weather/lc.svg",
      jsonValuePath: "[0].the_temp"
    },
    {
      endpointUrlHttps: "random-data-api.com/api/name/random_name",
      value: "Hannelore",
      description: "Random first name",
      iconUrlHttps: "res.cloudinary.com/manoylo/image/upload/v1583284399/manoylome_dhxyc0.png",
      jsonValuePath: "first_name"
    },
    {
      endpointUrlHttps: "random-data-api.com/api/name/random_name",
      value: "7236345754",
      description: "Random positive integer",
      iconUrlHttps: "res.cloudinary.com/manoylo/image/upload/v1583284399/manoylome_dhxyc0.png",
      jsonValuePath: "number"
    }
  ]);
  
  const refresh = useCallback(() => {

    if (!metaDataPubKey) {
      return;
    }

    connection.getAccountInfo(metaDataPubKey).then((metaData) => {
      if (!metaData) return;
      const dat = metaData.data.slice(0, 32);
      const apiData = new PublicKey(dat);

      connection.getAccountInfo(apiData).then((apiData) => {
        if (!apiData) return;
        const endpointUrlHttps = apiData.data.slice(0, 99).toString('utf8');
        const description = apiData.data.slice(100, 199).toString('utf8');
        const value = apiData.data.slice(200, 299).toString('utf8');
        const jsonValuePath = apiData.data.slice(300, 399).toString('utf8');
        const iconUrlHttps = apiData.data.slice(400, 499).toString('utf8');

        setOracles([{ 
          endpointUrlHttps,
          description,
          value,
          jsonValuePath,
          iconUrlHttps 
        }]);

        notify({
          message: "Loaded from Solana: " + description,
          type: "success",
        });
      });
    });
  }, [metaDataPubKey, connection]);

  // 
  const cards = oracles.map(oracle => {
    return (
      <Col span={6}>
        <Card
          key={oracle.description}
          hoverable
          cover={<img alt={oracle.description} src={"https://" + oracle.iconUrlHttps} />}
        >
          <Meta title={oracle.description} />
          <h1>{oracle.value}</h1>
          <h5>Source: <a href={"https://" + oracle.endpointUrlHttps}>{oracle.endpointUrlHttps}</a></h5>
          <h6>Value path: {oracle.jsonValuePath}</h6>
        </Card>
      </Col>
    )
  })
  return (
    <>
      <h1>Browse Oracles</h1>
      <Button onClick={refresh}>Sync from Solana</Button>
      <Row align="middle">
          {cards}
      </Row>
    </>
  )
};
