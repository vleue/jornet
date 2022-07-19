import React, { Component } from "react";
import { Col, Container, Row } from "react-bootstrap";

class Landing extends Component {
    render() {
        return (
            <Container fluid="lg">
                <Row>
                    <Col>Jornet is the social game server made for game jams!</Col>
                </Row>
            </Container>
        );
    }
}

export default Landing;