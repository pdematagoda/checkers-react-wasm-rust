import { lazy, Suspense } from "react";

const App = () => {

    const CheckersBoard = lazy(() => import('./CheckersBoard'));

    return <Suspense fallback={<div>Loading stuff</div>}>
        <CheckersBoard />
    </Suspense>;
};

export default App;