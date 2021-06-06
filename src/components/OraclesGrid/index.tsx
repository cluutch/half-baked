import { Card, Col, Row  } from "antd";
import React from "react";

const { Meta } = Card;

export const OraclesGrid = () => {
  const oracles = [
    {
      endpointUrlHttps: "api.cluutch.io/v2/daily\?date\=2021-06-05",
      lastValue: "$159.00",
      timestamp: 1234567,
      description: "Price for 1oz of weed",
      iconUrlHttps: "cluutch.io/clutch.svg"
    },
    {
      endpointUrlHttps: "datausa.io/api/data?drilldowns=Nation&measures=Population&year=latest",
      lastValue: "327167439",
      timestamp: 1234567,
      description: "Population of USA",
      iconUrlHttps: "datausa.io/images/logo_sm.png"
    },
    {
      endpointUrlHttps: "random-data-api.com/api/name/random_name",
      lastValue: "Hannelore",
      timestamp: 1234567,
      description: "Random first name",
      iconUrlHttps: "res.cloudinary.com/manoylo/image/upload/v1583284399/manoylome_dhxyc0.png"
    },
    {
      endpointUrlHttps: "random-data-api.com/api/name/random_name",
      lastValue: "7236345754",
      timestamp: 1234567,
      description: "Random positive integer",
      iconUrlHttps: "res.cloudinary.com/manoylo/image/upload/v1583284399/manoylome_dhxyc0.png"
    },
    {
      endpointUrlHttps: "www.metaweather.com/api/location/968019",
      lastValue: "20210606: 18.53C",
      timestamp: 1234567,
      description: "Temperature in Brussels",
      iconUrlHttps: "www.metaweather.com/static/img/weather/lc.svg"
    }
  ]

  const cards = oracles.map(oracle => {
    return (
      <Col span={6}>
        <Card
          key={oracle.description}
          hoverable
          cover={<img alt={oracle.description} src={"https://" + oracle.iconUrlHttps} />}
        >
          <Meta title={oracle.description} />
          <h1>{oracle.lastValue}</h1>
          <h5>Source: <a href={"https://" + oracle.endpointUrlHttps}>{oracle.endpointUrlHttps}</a></h5>
          <h6>Last fetched: {oracle.timestamp}</h6>
        </Card>
      </Col>

    )
  })
  return (
    <>
      <h1>Browse Oracles</h1>
      <Row align="middle">
          {cards}
      </Row>
      
    </>
  )
};
