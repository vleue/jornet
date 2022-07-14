import React, { Component } from "react";
import { NavigateFunction, useNavigate } from "react-router-dom";


type User = {
    uuid: String,
    github_login?: String
}
type DashboardProps = {
    token?: string;
    navigate?: NavigateFunction;
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
                this.setState({ user: { uuid: data.admin.id, github_login: data.github?.login } });
            });
    }

    render() {
        if (this.props.token === undefined) {
            setTimeout(() => this.props.navigate!("/connect"), 200);
            return (
                <div>
                    <div>You are disconnected, redirecting to login screen</div>
                </div>
            );
        }
        if (this.state.user === undefined) {
            return (
                <div>
                    Loading...
                </div>
            );
        }
        return (
            <div>
                <div>Hello</div>
                <div>Connected as {this.state.user?.uuid!}
                    <div>{this.state.user?.github_login === undefined ? (
                        <div>using UUID</div>
                    ) : (
                        <div>using GitHub account {this.state.user?.github_login!}</div>
                    )}
                    </div>
                </div>
            </div>
        );
    }
}

export default function Dashboard(props: DashboardProps) {
    return <DashboardInner {...props} navigate={useNavigate()} />;
};
