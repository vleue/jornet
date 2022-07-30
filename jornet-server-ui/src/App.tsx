import React, { Component } from "react";
import {
  Route,
  BrowserRouter as Router,
  Routes,
} from "react-router-dom";
import Landing from "./pages/landing";
import Connect from "./pages/connect";
import Dashboard from "./pages/dashboard";
import Leaderboard from "./pages/leaderboard";
import 'bootstrap/dist/css/bootstrap.min.css';
import Container from 'react-bootstrap/Container';
import Navbar from 'react-bootstrap/Navbar';
import Nav from "react-bootstrap/Nav";
import { LinkContainer } from 'react-router-bootstrap'

type AppProps = {};
type AppState = {
  token?: string;
  login_info?: string;
};

class App extends Component<AppProps, AppState> {
  state: AppState = {
    token: undefined,
    login_info: undefined,
  };
  render() {
    return (
      <div className="App" >
        <Router>
          <Navbar bg="info" variant="dark">
            <Container>
              <LinkContainer to="/">
                <Navbar.Brand>
                  <h1>
                    <img
                      alt=""
                      src="logo-50.png"
                      width="50"
                      height="50"
                      className="d-inline-block align-top"
                    />{' '}
                    Jornet
                  </h1>
                </Navbar.Brand>
              </LinkContainer>
              <Navbar.Toggle />
              <Navbar.Collapse className="justify-content-end">
                <Navbar.Text>
                  {this.state.login_info === undefined ? (
                    <LinkContainer to="/connect">
                      <Nav.Link>Connect...</Nav.Link>
                    </LinkContainer>) : (
                    <LinkContainer to="/dashboard">
                      <Nav.Link>connected as {this.state.login_info}</Nav.Link>
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
                <Route path="/dashboard" element={<Dashboard token={this.state.token} setLoginInfo={this.setLoginInfo} setToken={this.setToken} />} />
                <Route path="/leaderboard/:leaderboardId" element={<Leaderboard />} />
              </Routes>
            </div></div>
        </Router>
      </div>
    );
  }
  setToken = (token?: string) => {
    this.setState({ token: token });
  }
  setLoginInfo = (login_info?: string) => {
    this.setState({ login_info: login_info });
  }
}

export default App;
