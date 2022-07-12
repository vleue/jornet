import React, { Component } from "react";
import { NavigateFunction, NavLink, useNavigate } from "react-router-dom";


type DashboardProps = {
    token?: string;
    navigate?: NavigateFunction;
};
type DashboardState = {};


class Dashboard extends Component<DashboardProps, DashboardState> {
    componentDidMount() {
        if (this.props.token === undefined) {
            return setTimeout(() => this.props.navigate!("/connect"));
        }
        fetch("/api/admin/whoami", { headers: { Authorization: 'Bearer ' + this.props.token! } })
            .then(response => response.json())
            .then(data => console.log(data));
    }

    render() {
        if (this.props.token === undefined) {
            return (
                <div>
                    <div>You are disconnected, redirecting to login screen</div>
                </div>
            );
        }
        return (
            <div>
                <div>Hello</div>
            </div>
        );
    }
}

export default function (props: DashboardProps) {
    const navigate = useNavigate();

    return <Dashboard {...props} navigate={navigate} />;
};
