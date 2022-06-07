import { useState } from "react";
import { doMove, generateBoard, Unit, initialiseEngine, Board, copyBoard, WinningSide } from "wasm-ai-thingo";
import Grid from "./Grid";

initialiseEngine();

const CheckersBoard = () => {
    const [board, setBoard] = useState(generateBoard());
    const [lastBoard, setLastBoard] = useState<Board | null>(null);

    const onMove = (unit: Unit, x: number, y: number) => {
        if (lastBoard !== null) {
            lastBoard.free();
        }

        setLastBoard(copyBoard(board));

        const newBoard = doMove(board, unit, x, y);

        setBoard(newBoard);
    };

    const onUndo = () => {
        if (lastBoard !== null) {
            setBoard(copyBoard(lastBoard));

            lastBoard.free();
            setLastBoard(null);
        }

        board.free();
    };

    const onReset = () => {
        setBoard(generateBoard());
        board.free();
    };

    const gameStateString = board.winning_side === WinningSide.None ? 'Active' :
        (board.winning_side === WinningSide.White ? 'White has won' : 'Black has won');

    return (<div>
        <button disabled={lastBoard === null} onClick={onUndo} style={{ marginBottom: 10 }}>Undo</button>
        <label style={{ marginLeft: 10 }}>Game State: {gameStateString}</label>
        <button onClick={onReset} style={{ marginBottom: 10, marginLeft: 10 }}>Reset</button>
        <Grid board={board} onMove={onMove} />
    </div>);
};

export {
    CheckersBoard
};

export default CheckersBoard;