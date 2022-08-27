import React, { Component } from "react";
import { Alert, Button, Col, Container, FloatingLabel, Form, InputGroup, Nav, Row, Table } from "react-bootstrap";
import { NavigateFunction, useNavigate, useSearchParams } from "react-router-dom";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faClipboard, faClipboardCheck } from '@fortawesome/free-solid-svg-icons'
import { LinkContainer } from "react-router-bootstrap";
import SyntaxHighlighter from "react-syntax-highlighter";
import { docco } from 'react-syntax-highlighter/dist/esm/styles/hljs';
import { CSSProperties } from "react";


type ClipboardHelperProps = {
    to_copy: string,
    style?: CSSProperties,
}

type ClipboardHelperState = {
    clicked: boolean,
}

class ClipboardHelper extends Component<ClipboardHelperProps, ClipboardHelperState> {
    state: ClipboardHelperState = {
        clicked: false
    };
    render() {
        if (this.state.clicked) {
            setTimeout(() => { this.setState({ clicked: false }) }, 2000);
            return (<FontAwesomeIcon
                icon={faClipboardCheck}
                onClick={() => {
                    navigator.clipboard.writeText(this.props.to_copy);
                    this.setState({ clicked: true })
                }}
                style={{
                    marginLeft: "0.5rem",
                    fontSize: "1.1rem",
                    color: "green",
                    ...this.props.style
                }}
            />
            );
        } else {
            return (<FontAwesomeIcon
                icon={faClipboard}
                onClick={() => {
                    navigator.clipboard.writeText(this.props.to_copy);
                    this.setState({ clicked: true })
                }}
                style={{
                    marginLeft: "0.5rem",
                    ...this.props.style
                }}
            />
            );
        }
    }
}

type User = {
    uuid: string,
    github_login?: string
}
type Leaderboard = {
    name: string,
    id: string,
    scores: number,
    key?: string,
}
type DashboardProps = {
    token?: string;
    navigate?: NavigateFunction;
    setLoginInfo: (uuid?: string) => void;
    setToken: (token?: string) => void;
    new_account?: boolean;
};
type DashboardState = {
    user?: User;
    new_leaderboard: string;
    leaderboards: Leaderboard[]
    new_leaderboard_data?: Leaderboard;
};


class DashboardInner extends Component<DashboardProps, DashboardState> {
    state: DashboardState = {
        leaderboards: [],
        new_leaderboard: "",
    };
    componentDidMount() {
        if (this.props.token === undefined) {
            return;
        }
        fetch("/api/v1/admin/whoami", { headers: { Authorization: 'Bearer ' + this.props.token! } })
            .then(response => response.json())
            .then(data => {
                if (data.github?.login === undefined) {
                    this.props.setLoginInfo(data.admin.id);
                } else {
                    this.props.setLoginInfo(data.github?.login);
                }
                this.setState({ user: { uuid: data.admin.id, github_login: data.github?.login } });
            }).catch(error => {
                this.props.setLoginInfo(undefined);
                this.props.setToken(undefined);
            });
        fetch("/api/v1/leaderboards", { headers: { Authorization: 'Bearer ' + this.props.token! } })
            .then(response => response.json())
            .then(data => {
                this.setState({ leaderboards: data });
            }).catch(error => {
                this.props.setLoginInfo(undefined);
                this.props.setToken(undefined);
            });
    }

    render() {
        if (this.props.token === undefined) {
            setTimeout(() => this.props.navigate!("/connect"), 200);
            return (
                <Container fluid="lg">
                    <Row>
                        <Col>You are disconnected, redirecting to login screen</Col>
                    </Row>
                </Container>
            );
        }
        if (this.state.user === undefined) {
            return (
                <Container fluid="lg">
                    <Row>
                        <Col>Loading...</Col>
                    </Row>
                </Container>
            );
        }
        return (
            <Container fluid="lg">
                <Row>
                    <Col>&nbsp;</Col>
                </Row>
                {this.props.new_account ? (
                    <Row>
                        <Col>
                            <Alert key="new_account" variant="warning" style={{ display: "flex" }}>
                                <div>You'll need to keep your account UUID to manage your dashboards: </div>
                                <div>&nbsp;</div>
                                <div className="font-monospace">{this.state.user.uuid}</div>
                                <ClipboardHelper to_copy={this.state.user!.uuid} />
                            </Alert>
                        </Col>
                    </Row>
                ) : (<></>)
                }
                <Row>
                    <Col>
                        <InputGroup>
                            <FloatingLabel
                                className="w-75"
                                controlId="new-leaderboard"
                                label="New Leaderboard Name"
                            >
                                <Form.Control
                                    type="text"
                                    placeholder="New Leaderboard Name"
                                    value={this.state.new_leaderboard}
                                    onChange={this.handleChangeNewLeaderboard}
                                />
                            </FloatingLabel>
                            <Button
                                variant="primary"
                                onClick={this.handleSubmitNewLeaderboard}
                                disabled={this.state.new_leaderboard === ""}
                            >
                                Create
                            </Button>
                        </InputGroup>
                    </Col>
                </Row>
                <Row>
                    <Col>
                        &nbsp;
                    </Col>
                </Row>
                {this.state.new_leaderboard_data !== undefined ? (
                    <Row>
                        <Col>
                            <Alert key="new_leaderboard" variant="warning">
                                <div style={{ display: "flex" }}>
                                    <div>You'll need to keep your new leaderboard key to use it: </div>
                                    <div>&nbsp;</div>
                                    <div className="font-monospace">{this.state.new_leaderboard_data.key}</div>
                                    <ClipboardHelper to_copy={this.state.new_leaderboard_data?.key!} />
                                </div>
                                <div>
                                    <div>Here is the code to setup your new leaderboard in Bevy: </div>
                                    <div style={{ display: "flex" }}>
                                        <SyntaxHighlighter language="rust" style={docco} customStyle={{ marginBottom: "0px" }}>
                                            {"app.add_plugin(JornetPlugin::with_leaderboard(\"" + this.state.new_leaderboard_data.id + "\", \"" + this.state.new_leaderboard_data.key! + "\"));"}
                                        </SyntaxHighlighter>
                                        <ClipboardHelper
                                            to_copy={"app.add_plugin(JornetPlugin::with_leaderboard(\"" + this.state.new_leaderboard_data?.id! + "\", \"" + this.state.new_leaderboard_data?.key! + "\"));"}
                                            style={{ marginTop: "0.5rem" }}
                                        />
                                    </div>
                                    <div>You should avoid exposing those ID/key in a public repository.</div>
                                </div>
                            </Alert>
                        </Col>
                    </Row>
                ) : (<></>)
                }
                <Row>
                    <Col>
                        <Table striped bordered hover>
                            <thead>
                                <tr>
                                    <th>Leaderboard</th>
                                    <th>Scores</th>
                                    <th>ID</th>
                                    <th></th>
                                </tr>
                            </thead>
                            <tbody>
                                {
                                    this.state.leaderboards.map((leaderboard, index) => {
                                        return <tr key={index}>
                                            <td>
                                                <LinkContainer to={`/leaderboard/${leaderboard.id}`}>
                                                    <Nav.Link>{leaderboard.name}</Nav.Link>
                                                </LinkContainer>
                                            </td>
                                            <td>{leaderboard.scores}</td>
                                            <td style={{ display: "flex" }}>
                                                <p className="font-monospace">{leaderboard.id}</p>
                                                <ClipboardHelper to_copy={leaderboard.id} />
                                            </td>
                                            <td>
                                                <Button
                                                    variant="primary"
                                                    data-leaderboard-id={leaderboard.id}
                                                    onClick={this.handleResetLeaderboard}
                                                >
                                                    Reset
                                                </Button>
                                            </td>
                                        </tr>
                                    })
                                }
                            </tbody>
                        </Table>
                    </Col>
                </Row>
            </Container >
        );
    }
    handleChangeNewLeaderboard = (event: React.ChangeEvent<HTMLInputElement>) => {
        this.setState({ new_leaderboard: event.target.value });
    }
    handleSubmitNewLeaderboard = (event: React.FormEvent) => {
        this.setState({ new_leaderboard: "" });
        const requestOptions = {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
                'Authorization': 'Bearer ' + this.props.token!
            },
            body: JSON.stringify({ name: this.state.new_leaderboard })
        };
        fetch('/api/v1/leaderboards', requestOptions)
            .then(response => response.json())
            .then(data => {
                var leaderboards = this.state.leaderboards;
                leaderboards.push(data)
                this.setState({ leaderboards: leaderboards });
                this.setState({ new_leaderboard_data: data });
            }).catch(error => {
                this.props.setLoginInfo(undefined);
                this.props.setToken(undefined);
            });
        event.preventDefault();
    }
    handleResetLeaderboard = (event: React.FormEvent) => {
        let leaderboardId = event.currentTarget.getAttribute('data-leaderboard-id');
        const requestOptions = {
            method: 'DELETE',
            headers: {
                'Content-Type': 'application/json',
                'Authorization': 'Bearer ' + this.props.token!
            },
        };
        fetch('/api/v1/scores/' + leaderboardId, requestOptions)
            .then(response => response.json())
            .then(data => {
                // TODO
            }).catch(error => {
                this.props.setLoginInfo(undefined);
                this.props.setToken(undefined);
            });
        event.preventDefault();
    }
}

export default function Dashboard(props: DashboardProps) {
    const searchParams = useSearchParams()[0];
    let new_account = searchParams.get("new_account");
    return <DashboardInner {...props} navigate={useNavigate()} new_account={new_account === ""} />;
};
