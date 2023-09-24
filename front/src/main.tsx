import React from "react";
import ReactDOM from "react-dom/client";
import "./index.css";
import App from "./components/App";

// Store access token in local storage
const token = location.search.match(/\?token=([^&]+)/)?.[1];
if (token) localStorage.setItem("anlg-token", token);

ReactDOM.createRoot(document.getElementById("root")!).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>
);
