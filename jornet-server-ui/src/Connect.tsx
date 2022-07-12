import { throws } from "assert";
import React, { Component } from "react";
import { NavigateFunction, useNavigate } from "react-router-dom";
import validator from "validator";

type ConnectProps = {
    navigate?: NavigateFunction;
    setToken: (token: string) => void;
};
type ConnectState = {
    github_app_id?: string;
    uuid: string;
};

class Connect extends Component<ConnectProps, ConnectState> {
    state: ConnectState = {
        github_app_id: undefined,
        uuid: "",
    };
    componentDidMount() {
        fetch('/api/config/oauth')
            .then(response => response.json())
            .then(data => this.setState({ github_app_id: data.github_app_id }));
    }
    render() {
        return (
            <div>
                {this.state.github_app_id === undefined ? (
                    <div>Connect using GitHub (disabled)</div>
                ) : (
                    <a href={`https://github.com/login/oauth/authorize?client_id=${this.state.github_app_id}"`}>Connect using GitHub</a>
                )}
                <form onSubmit={this.handleSubmit}>
                    <label>UUID:
                        <input type="text" value={this.state.uuid} onChange={this.handleChange} />
                    </label>
                    <input type="submit" value="Authenticate with UUID" disabled={!validator.isUUID(this.state.uuid)} />
                </form>
            </div >
        );
    }
    handleChange = (event: React.ChangeEvent<HTMLInputElement>) => {
        this.setState({ uuid: event.target.value });
    }
    handleSubmit = (event: React.FormEvent) => {
        const requestOptions = {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ uuid: this.state.uuid })
        };
        fetch('/oauth/by_uuid', requestOptions)
            .then(response => response.json())
            .then(data => {
                this.props.setToken(data.token);
                this.props.navigate!("/dashboard");
            })
        event.preventDefault();
    }
}

export default function (props: ConnectProps) {
    const navigate = useNavigate();

    return <Connect {...props} navigate={navigate} />;
};
