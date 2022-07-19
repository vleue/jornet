import React, { Component } from "react";
import { NavigateFunction, useNavigate, useSearchParams } from "react-router-dom";
import validator from "validator";
import { v4 as uuidv4 } from "uuid";
import { Button, Col, Container, FloatingLabel, Form, InputGroup, Row } from "react-bootstrap";

type ConnectProps = {
    navigate?: NavigateFunction;
    searchParams?: URLSearchParams;
    setToken: (token?: string) => void;
};
type ConnectState = {
    github_app_id?: string;
    uuid: string;
    is_from_callback: boolean;
    error?: string;
};

class ConnectInner extends Component<ConnectProps, ConnectState> {
    state: ConnectState = {
        github_app_id: undefined,
        uuid: "",
        is_from_callback: false,
    };
    componentDidMount() {
        fetch('/api/config/oauth')
            .then(response => response.json())
            .then(data => this.setState({ github_app_id: data.github_app_id }));
        let code = this.props.searchParams!.get("code");
        if (code !== null) {
            this.setState({ is_from_callback: true })
            fetch(`/oauth/callback?code=${code}`)
                .then(response => response.json())
                .then(data => {
                    this.props.setToken(data.token);
                    setTimeout(() => this.props.navigate!("/dashboard"));
                })

        }
    }
    render() {
        if (this.state.is_from_callback) {
            return (
                <div>
                    Connecting with GitHub...
                </div>
            )
        }
        return (
            <Container fluid="lg">
                <Row>
                    <Col>
                        &nbsp;
                    </Col>
                </Row>
                <Row>
                    <Col>
                        {this.state.error !== undefined ? (
                            <div>{this.state.error}</div>
                        ) : (
                            <div></div>
                        )}
                    </Col>
                </Row>
                <Row>
                    <Col sm={6}>
                        <InputGroup>
                            <FloatingLabel
                                controlId="uuid"
                                className="w-75"
                                label="Your UUID"
                            >
                                <Form.Control
                                    type="text"
                                    placeholder="Your UUID"
                                    value={this.state.uuid}
                                    onChange={this.handleChange}
                                />
                            </FloatingLabel>
                            <Button
                                variant="primary"
                                onClick={this.handleSubmit}
                                disabled={!validator.isUUID(this.state.uuid)}
                            >
                                Connect
                            </Button>
                        </InputGroup>
                    </Col>
                    <Col>
                        <Button
                            style={{ padding: "16px" }}
                            className="w-75"
                            variant="info"
                            onClick={this.handleSubmit}
                            disabled={this.state.uuid !== ""}
                        >
                            New Account
                        </Button>
                    </Col>
                    <Col>
                        {this.state.github_app_id === undefined ? (
                            <Button
                                style={{ padding: "16px" }}
                                className="w-75"
                                variant="success"
                                disabled={true}
                            >
                                Connect using GitHub (disabled)
                            </Button>

                        ) : (
                            <a href={`https://github.com/login/oauth/authorize?client_id=${this.state.github_app_id}`}>
                                <Button
                                    style={{ padding: "16px" }}
                                    className="w-75"
                                    variant="success"
                                >
                                    Connect using GitHub
                                </Button>
                            </a>
                        )}
                    </Col>
                </Row>
            </Container >
        );
    }
    handleChange = (event: React.ChangeEvent<HTMLInputElement>) => {
        this.setState({ uuid: event.target.value, error: undefined });
    }
    handleSubmit = (event: React.FormEvent) => {
        let uuid = this.state.uuid !== "" ? this.state.uuid : uuidv4();
        const requestOptions = {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ uuid: uuid })
        };
        this.setState({ error: undefined });
        fetch('/oauth/by_uuid', requestOptions)
            .then(response => response.json())
            .then(data => {
                this.props.setToken(data.token);
                this.props.navigate!("/dashboard");
            })
            .catch(reason => {
                this.setState({ uuid: "", error: "Error connecting with this UUID, try another." });
            })
        event.preventDefault();
    }
}

export default function Connect(props: ConnectProps) {
    const searchParams = useSearchParams()[0];
    return <ConnectInner {...props} navigate={useNavigate()} searchParams={searchParams} />;
};
