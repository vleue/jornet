import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faHeart } from "@fortawesome/free-solid-svg-icons";
import { faGithub } from "@fortawesome/free-brands-svg-icons"
import React, { Component } from "react";
import { Button, Col, Container, Nav, Row, Tab, Tabs } from "react-bootstrap";
import { LinkContainer } from "react-router-bootstrap";
import Leaderboard from "./leaderboard";

class Landing extends Component {
    render() {
        return (
            <>
                <Container fluid="lg">
                    <Row>
                        <Col>&nbsp;</Col>
                    </Row>
                    <Row>
                        <Col xs={{ span: 7, offset: 2 }}>
                            <h1 className="display-4 text-center">
                                Jornet is the social game server made for game jams!
                            </h1>
                        </Col>
                        <Col xs={2}>
                            <img
                                alt="logo"
                                src="/logo-1400.png"
                                width="130"
                                height="130"
                                className="d-inline-block align-top"
                            />
                        </Col>
                    </Row>
                    <Row><Col>&nbsp;</Col></Row>
                    <Row>
                        <Col className="text-center" xs={{ span: 2, offset: 4 }}>
                            <a
                                href="https://github.com/sponsors/mockersf"
                                target="_blank"
                                rel="noreferrer"
                            >
                                <Button variant="success">
                                    <FontAwesomeIcon icon={faHeart} color="red" />{' '}
                                    Sponsor Me
                                </Button>
                            </a>
                        </Col>
                        <Col className="text-center" xs={{ span: 2 }}>
                            <a
                                href="https://github.com/vleue/jornet"
                                target="_blank"
                                rel="noreferrer"
                            >
                                <Button variant="primary">
                                    <FontAwesomeIcon icon={faGithub} />{' '}
                                    View on GitHub
                                </Button>
                            </a>
                        </Col>
                    </Row>
                    <Row><Col>&nbsp;</Col></Row>
                    <Row>
                        <Col>
                            <Tabs
                                defaultActiveKey="wasm-demo"
                                id="landing-tabs"
                                className="mb-3"
                            >
                                <Tab eventKey="wasm-demo" title="Bevy Demo">
                                    <canvas id="demo-leaderboard"></canvas>
                                </Tab>
                                <Tab eventKey="view-leaderboard" title="View Leaderboard">
                                    <Leaderboard leaderboardId="fb0bbe22-b047-494d-9519-1d36668fa5bc" />
                                </Tab>
                            </Tabs>
                        </Col>
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
