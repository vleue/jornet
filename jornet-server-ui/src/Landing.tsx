import React, { Component } from "react";
import { NavLink } from "react-router-dom";

class Landing extends Component {
    render() {
        return (
            <div>
                <div>Jornet is the social game server made for game jams!</div>
                <NavLink to="/connect">Connect to Admin Dashboard</NavLink>
            </div>
        );
    }
}

export default Landing;