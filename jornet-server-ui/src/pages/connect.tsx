import React, { Component } from "react";
import { NavigateFunction, useNavigate, useSearchParams } from "react-router-dom";
import validator from "validator";
import { v4 as uuidv4 } from "uuid";

type ConnectProps = {
    navigate?: NavigateFunction;
    searchParams?: URLSearchParams;
    setToken: (token: string) => void;
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
            <div>
                {this.state.github_app_id === undefined ? (
                    <div>Connect using GitHub (disabled)</div>
                ) : (
                    <a href={`https://github.com/login/oauth/authorize?client_id=${this.state.github_app_id}`}>Connect using GitHub</a>
                )}
                <hr />
                Alternatively, you can connect using an UUID, in which case you'll need to remember it as it will be the only way to connect.
                {this.state.error !== undefined ? (
                    <div>{this.state.error}</div>
                ) : (
                    <div></div>
                )}
                <form onSubmit={this.handleSubmit}>
                    <label>
                        <input type="text" value={this.state.uuid} onChange={this.handleChange} placeholder="UUID" />
                    </label>
                    <input type="submit" value="Connect using UUID" disabled={!validator.isUUID(this.state.uuid)} />
                </form>
                <form onSubmit={this.handleSubmit}>
                    <input type="submit" value="New account" />
                </form>
            </div >
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
