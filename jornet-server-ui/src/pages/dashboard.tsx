import React, { Component } from "react";
import { Col, Container, Row } from "react-bootstrap";
import { NavigateFunction, useNavigate } from "react-router-dom";


type User = {
    uuid: String,
    github_login?: String
}
type DashboardProps = {
    token?: string;
    navigate?: NavigateFunction;
    setUuid: (uuid?: string) => void;
    setToken: (token?: string) => void;
};
type DashboardState = {
    user?: User;
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
                this.props.setUuid(data.admin.id);
                this.setState({ user: { uuid: data.admin.id, github_login: data.github?.login } });
            }).catch(error => {
                this.props.setUuid(undefined);
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
                    <Col>Hello!</Col>
                </Row>
                <Row>
                    {this.state.user?.github_login === undefined ? (
                        <Col>using UUID</Col>
                    ) : (
                        <Col>using GitHub account {this.state.user?.github_login!}</Col>
                    )}
                </Row>
            </Container>
        );
    }
}

export default function Dashboard(props: DashboardProps) {
    return <DashboardInner {...props} navigate={useNavigate()} />;
};
