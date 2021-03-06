import React, { Component } from "react";
import { Alert, Button, Col, Container, FloatingLabel, Form, InputGroup, Nav, Row, Table } from "react-bootstrap";
import { NavigateFunction, useNavigate, useSearchParams } from "react-router-dom";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faClipboard } from '@fortawesome/free-solid-svg-icons'
import { LinkContainer } from "react-router-bootstrap";


type User = {
    uuid: string,
    github_login?: string
}
type Leaderboard = {
    name: string,
    id: string,
    scores: number,
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
        fetch("/api/admin/whoami", { headers: { Authorization: 'Bearer ' + this.props.token! } })
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
        fetch("/api/leaderboards", { headers: { Authorization: 'Bearer ' + this.props.token! } })
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
                                <div>You'll need to keep your account UUID to reconnect with it: </div>
                                <div>&nbsp;</div>
                                <div className="font-monospace">{this.state.user.uuid}</div>
                                <FontAwesomeIcon icon={faClipboard} onClick={() => { navigator.clipboard.writeText(this.state.user!.uuid) }} style={{ marginLeft: "0.5rem" }} />
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
                <Row>
                    <Col>
                        <Table striped bordered hover>
                            <thead>
                                <tr>
                                    <th>Leaderboard</th>
                                    <th>Scores</th>
                                    <th>ID</th>
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
                                                <FontAwesomeIcon icon={faClipboard} onClick={() => { navigator.clipboard.writeText(leaderboard.id) }} style={{ marginLeft: "0.5rem" }} />
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
        fetch('/api/leaderboards', requestOptions)
            .then(response => response.json())
            .then(data => {
                var leaderboards = this.state.leaderboards;
                leaderboards.push(data)
                this.setState({ leaderboards: leaderboards });
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
