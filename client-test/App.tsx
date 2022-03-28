import { useEffect, useState } from "react";
import { CheckersBoard } from "./CheckersBoard";

const App = () => {
    const [isLoading, setIsLoading] = useState(true);
    const [logicPromise] = useState(import("wasm-ai-thingo"));

    useEffect(() => {
        logicPromise.then(() => {
            setIsLoading(false);
        });
    });

    if (isLoading) {
        return <div>Loading stuff</div>;
    }

    return <CheckersBoard />;
};

export default App;