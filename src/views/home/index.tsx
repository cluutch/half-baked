import { Button, Col, Row } from "antd";
import React from "react";
import { Link } from "react-router-dom";
import { OraclesGrid } from "../../components/OraclesGrid";

export const HomeView = () => {

  return (
    <Row gutter={[16, 16]} align="middle">
      <Col span={24}>
        <Row align="middle">
          <Col span={16}>
            <h1>Oracle Library</h1>
            <h3>Browse existing off-chaing data, now available on Solana. Create new data feeds.</h3>
          </Col>
          <Col span={8}>
            <Link to="/oracles/new">
              <Button>Add New Oracle</Button>
            </Link>

          </Col>
        </Row>
      </Col>

      <Col span={24}>
        <OraclesGrid />
      </Col>
      
      <Col span={24}>
        built by cluutch.io
      </Col>
    </Row>
  );
};
