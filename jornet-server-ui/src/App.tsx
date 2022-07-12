import React from "react";
import {
  Route,
  BrowserRouter as Router,
  Routes
} from "react-router-dom";
import Landing from "./Landing";
import Connect from "./Connect";

function App() {
  return (
    <div className="App">
      <Router>
        <div>
          <h1>Jornet</h1>
          <div className="content">
            <Routes>
              <Route path="/" element={<Landing />} />
              <Route path="/connect" element={<Connect />} />
            </Routes>
          </div></div>
      </Router>
    </div>
  );
}

export default App;
