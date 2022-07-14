import React, { Component } from "react";
import { NavigateFunction, useNavigate, useSearchParams } from "react-router-dom";
import validator from "validator";

type ConnectProps = {
    navigate?: NavigateFunction;
    searchParams?: URLSearchParams;
    setToken: (token: string) => void;
};
type ConnectState = {
    github_app_id?: string;
    uuid: string;
    is_from_callback: boolean;
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
            <div>
                {this.state.github_app_id === undefined ? (
                    <div>Connect using GitHub (disabled)</div>
                ) : (
                    <a href={`https://github.com/login/oauth/authorize?client_id=${this.state.github_app_id}`}>Connect using GitHub</a>
                )}
                <hr />
                <form onSubmit={this.handleSubmit}>
                    <label>UUID:
                        <input type="text" value={this.state.uuid} onChange={this.handleChange} />
                    </label>
                    <input type="submit" value="Connect using UUID" disabled={!validator.isUUID(this.state.uuid)} />
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

export default function Connect(props: ConnectProps) {
    const searchParams = useSearchParams()[0];
    return <ConnectInner {...props} navigate={useNavigate()} searchParams={searchParams} />;
};
