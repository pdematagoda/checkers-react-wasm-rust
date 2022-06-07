import { StrictMode } from "react";
import ReactDOM from "react-dom";
import App from "./App";

window.addEventListener('load', () => {
    ReactDOM.render(<StrictMode><App /></StrictMode>, document.getElementById('root'));
});
