import { useState } from "react";
import { Coordinate, doMove, generateBoard, Unit, initialiseEngine } from "wasm-ai-thingo";
import Grid from "./Grid";

initialiseEngine();

const CheckersBoard = () => {
    const [board, setBoard] = useState(generateBoard());

    const onMove = (unit: Unit, x: number, y: number) => {
        const newBoard = doMove(board, unit, x, y);

        setBoard(newBoard);
    };

    return (<div>
        <Grid board={board} onMove={onMove} />
    </div>);
};

export {
    CheckersBoard
};

export default CheckersBoard;