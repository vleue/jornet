import React, { Component } from "react";
import { Col, Container, Row } from "react-bootstrap";

class Landing extends Component {
    render() {
        return (
            <Container fluid="lg">
                <Row>
                    <Col>&nbsp;</Col>
                </Row>
                <Row>
                    <Col></Col>
                    <Col xs={10}>
                        <big>
                            Jornet is the social game server made for game jams!
                        </big>
                    </Col>
                    <Col></Col>
                </Row>
            </Container>
        );
    }
}

export default Landing;