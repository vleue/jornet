import React, { Component } from "react";
import {
  Route,
  BrowserRouter as Router,
  Routes
} from "react-router-dom";
import Landing from "./Landing";
import Connect from "./Connect";
import Dashboard from "./Dashboard";


type AppProps = {};
type AppState = {
  token?: string;
};

class App extends Component<AppProps, AppState> {
  state: AppState = {
    token: undefined,
  };
  render() {
    return (
      <div className="App" >
        <Router>
          <div>
            <h1>Jornet</h1>
            <div className="content">
              <Routes>
                <Route path="/" element={<Landing />} />
                <Route path="/connect" element={<Connect setToken={this.setToken} />} />
                <Route path="/dashboard" element={<Dashboard token={this.state.token} />} />
              </Routes>
            </div></div>
        </Router>
      </div>
    );
  }
  setToken = (token: string) => {
    this.setState({ token: token });
  }
}

export default App;
