import React, { Component } from "react";
import { Link } from "react-router-dom";

class Landing extends Component {
    render() {
        return (
            <div>
                <div>Jornet is the social game server made for game jams!</div>
                <Link to="/connect">Connect to Admin Dashboard</Link>
            </div>
        );
    }
}

export default Landing;