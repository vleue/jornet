import React, { Component } from "react";
import { Col, Container, Row } from "react-bootstrap";

class Landing extends Component {
    render() {
        return (
            <>
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
                    <Row>
                        <Col>&nbsp;</Col>
                    </Row>
                    <Row>
                        <Col>
                            <a
                                href="https://jornet.vleue.com/leaderboard/fb0bbe22-b047-494d-9519-1d36668fa5bc"
                                target="_blank"
                                rel="noreferrer"
                            >
                                View the example leaderboard
                            </a>
                        </Col>
                    </Row>
                    <Row>
                        <Col><canvas id="demo-leaderboard"></canvas></Col>
                    </Row>
                </Container>

            </>
        );
    }
    componentDidMount() {
        const script = document.createElement('script');
        script.type = "module";
        script.textContent = "import init from './demo_leaderboard.js'; init();";
        document.body.appendChild(script);
    };
}

export default Landing;