import React, { Component } from "react";
import {
  Route,
  BrowserRouter as Router,
  Routes,
} from "react-router-dom";
import Landing from "./pages/landing";
import Connect from "./pages/connect";
import Dashboard from "./pages/dashboard";

import 'bootstrap/dist/css/bootstrap.min.css';
import Container from 'react-bootstrap/Container';
import Navbar from 'react-bootstrap/Navbar';
import Nav from "react-bootstrap/Nav";
import { LinkContainer } from 'react-router-bootstrap'

type AppProps = {};
type AppState = {
  token?: string;
  uuid?: string;
};

class App extends Component<AppProps, AppState> {
  state: AppState = {
    token: undefined,
    uuid: undefined,
  };
  render() {
    return (
      <div className="App" >
        <Router>
          <Navbar bg="dark" variant="dark">
            <Container>
              <LinkContainer to="/">
                <Navbar.Brand>
                  Jornet
                </Navbar.Brand>
              </LinkContainer>
              <Navbar.Toggle />
              <Navbar.Collapse className="justify-content-end">
                <Navbar.Text>
                  {this.state.uuid === undefined ? (
                    <LinkContainer to="/connect">
                      <Nav.Link><small>Connect...</small></Nav.Link>
                    </LinkContainer>) : (
                    <LinkContainer to="/dashboard">
                      <Nav.Link><small>connected as {this.state.uuid}</small></Nav.Link>
                    </LinkContainer>
                  )}
                </Navbar.Text>
              </Navbar.Collapse>
            </Container>
          </Navbar>
          <div>
            <div className="content">
              <Routes>
                <Route path="/" element={<Landing />} />
                <Route path="/connect" element={<Connect setToken={this.setToken} />} />
                <Route path="/dashboard" element={<Dashboard token={this.state.token} setUuid={this.setUuid} setToken={this.setToken} />} />
              </Routes>
            </div></div>
        </Router>
      </div>
    );
  }
  setToken = (token?: string) => {
    this.setState({ token: token });
  }
  setUuid = (uuid?: string) => {
    this.setState({ uuid: uuid });
  }
}

export default App;
