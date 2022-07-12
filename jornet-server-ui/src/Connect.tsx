import React, { Component } from "react";
import validator from "validator";

type ConnectProps = {};
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
        event.preventDefault();
    }
}

export default Connect;