import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faHeart } from "@fortawesome/free-solid-svg-icons";
import { faGithub } from "@fortawesome/free-brands-svg-icons"
import React, { Component } from "react";
import { Button, Col, Container, Row, Tab, Tabs } from "react-bootstrap";
import SyntaxHighlighter from 'react-syntax-highlighter';
import Leaderboard from "./leaderboard";
import { atomOneDarkReasonable } from 'react-syntax-highlighter/dist/esm/styles/hljs';

class BevyIntegration extends Component {
    render() {
        return (
            <div>
                <div>
                    View the full example on <a href="https://github.com/vleue/jornet/blob/main/bevy-jornet/examples/whac-a-square.rs" target="_blank" rel="noreferrer">GitHub</a>
                </div>
                <hr />
                <div>
                    Add the dependency.
                </div>
                <SyntaxHighlighter language="toml" style={atomOneDarkReasonable}>
                    bevy_jornet = "0.1"
                </SyntaxHighlighter>
                <hr />
                <div>
                    Add the plugin, specifying your settings.
                </div>
                <SyntaxHighlighter language="rust" style={atomOneDarkReasonable}>
                    {"app.add_plugins(JornetPlugin::with_leaderboard(id, key));"}
                </SyntaxHighlighter>
                <hr />
                <div>
                    Create a player if you don't already have one.
                </div>
                <SyntaxHighlighter language="rust" style={atomOneDarkReasonable}>
                    {"fn leaderboard_setup(mut leaderboard: ResMut<Leaderboard>) {\n\
    // `None` will create a new user with a random name\n\
    leaderboard.create_player(None);\n\
}"}
                </SyntaxHighlighter>
                <hr />
                <div>
                    Save a new score for the current user.
                </div>
                <SyntaxHighlighter language="rust" style={atomOneDarkReasonable}>
                    {"fn save_score(leaderboard: Res<Leaderboard>) {\n\
    leaderboard.send_score(10.0);\n\
}"}
                </SyntaxHighlighter>
                <hr />
                <div>
                    Refresh the leaderboard. This is asynchrone. The Leaderboard resource will be marked as changed once it has been refreshed.
                </div>
                <SyntaxHighlighter language="rust" style={atomOneDarkReasonable}>
                    {"fn refresh_leaderboard(leaderboard: Res<Leaderboard>) {\n\
    leaderboard.refresh_leaderboard();\n\
}"}
                </SyntaxHighlighter>
                <hr />
                <div>
                    Get the leaderboard, and display it how you want.
                </div>
                <SyntaxHighlighter language="rust" style={atomOneDarkReasonable}>
                    {"fn display_leaderboard(leaderboard: Res<Leaderboard>) {\n\
    if leaderboard.is_changed() {\n\
        for score in leaderboard.get_leaderboard() {\n\
            // display score\n\
        }\n\
    }\n\
}"}
                </SyntaxHighlighter>
            </div >
        );
    }
}

class About extends Component {
    render() {
        return (
            <div>
                Jornet is made for game jams. This means it will always be free for game jams, and will stay available. It works in WASM and on native platforms.<br />
                Please consider sponsoring me if you intend to use it for something more than a game jam.<hr />
                Data older than 3 months may be deleted.
            </div>
        );
    }
}
type LandingState = {
    refreshLeaderboard: number,
};
class Landing extends Component<{}, LandingState> {
    state: LandingState = {
        refreshLeaderboard: 0
    };
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
                                justify
                                onSelect={(k) => {
                                    if (k === 'view-leaderboard') {
                                        this.setState({ refreshLeaderboard: Date.now() });
                                    }
                                }}
                            >
                                <Tab eventKey="wasm-demo" title="Bevy Demo">
                                    <canvas id="demo-leaderboard"></canvas>
                                </Tab>
                                <Tab eventKey="view-leaderboard" title="View Leaderboard">
                                    <Leaderboard leaderboardId="edd47bbb-5d64-4a2b-84f5-b1fe9f31ec4c" refresh={this.state.refreshLeaderboard} />
                                </Tab>
                                <Tab eventKey="bevy-integration" title="Bevy Integration">
                                    <BevyIntegration />
                                </Tab>
                                <Tab eventKey="about" title="About">
                                    <About />
                                </Tab>
                            </Tabs>
                        </Col>
                    </Row>
                </Container>

            </>
        );
    }
    componentDidMount() {
        if (document.getElementById("loading-wasm") === null) {
            const script = document.createElement('script');
            script.type = "module";
            script.id = "loading-wasm";
            script.textContent = "import init from './demo_leaderboard.js'; init();";
            document.body.appendChild(script);
        }
    };
    componentWillUnmount() {
        document.getElementById("loading-wasm")?.remove();
    }
}

export default Landing;
