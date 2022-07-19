import React, { Component } from "react";
import { Button, Col, Container, FloatingLabel, Form, InputGroup, Row } from "react-bootstrap";
import { NavigateFunction, useNavigate } from "react-router-dom";


type User = {
    uuid: String,
    github_login?: String
}
type DashboardProps = {
    token?: string;
    navigate?: NavigateFunction;
    setLoginInfo: (uuid?: string) => void;
    setToken: (token?: string) => void;
};
type DashboardState = {
    user?: User;
    new_leaderboard?: string;
};


class DashboardInner extends Component<DashboardProps, DashboardState> {
    state: DashboardState = {
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
                                    onChange={this.handleChange}
                                />
                            </FloatingLabel>
                            <Button variant="primary" onClick={this.handleSubmit}
                            >
                                Create
                            </Button>
                        </InputGroup>
                    </Col>
                </Row>
            </Container>
        );
    }
    handleChange = (event: React.ChangeEvent<HTMLInputElement>) => {
        this.setState({ new_leaderboard: event.target.value });
    }
    handleSubmit = (event: React.FormEvent) => {
        console.log(this.state.new_leaderboard);
        // let uuid = this.state.uuid !== "" ? this.state.uuid : uuidv4();
        // const requestOptions = {
        //     method: 'POST',
        //     headers: { 'Content-Type': 'application/json' },
        //     body: JSON.stringify({ uuid: uuid })
        // };
        // this.setState({ error: undefined });
        // fetch('/oauth/by_uuid', requestOptions)
        //     .then(response => response.json())
        //     .then(data => {
        //         this.props.setToken(data.token);
        //         this.props.navigate!("/dashboard");
        //     })
        //     .catch(reason => {
        //         this.setState({ uuid: "", error: "Error connecting with this UUID, try another." });
        //     })
        // event.preventDefault();
    }
}

export default function Dashboard(props: DashboardProps) {
    return <DashboardInner {...props} navigate={useNavigate()} />;
};
