import { useState } from "react";
import { Board, Coordinate, Side, Unit, isValidBoardMove } from "wasm-ai-thingo"
import EmptyCell from "./EmptyCell";
import OccupiedCell from "./OccupiedCell";

interface GridProps {
    board: Board;
    onMove: (unit: Unit, x: number, y: number) => void;
}

const getKeyForCoordinate = (coordinate: Coordinate) => `${coordinate.x},${coordinate.y}`;

const addUnitToUnitDictionary = (dictonary: Map<string, Unit>, unit: Unit) => {
    if (unit.active) {
        dictonary.set(getKeyForCoordinate(unit.coordinate), unit);
    }
};

const addSideToUnitDictionary = (dictonary: Map<string, Unit>, side: Side) => {
    addUnitToUnitDictionary(dictonary, side.one);
    addUnitToUnitDictionary(dictonary, side.two);
    addUnitToUnitDictionary(dictonary, side.three);
    addUnitToUnitDictionary(dictonary, side.four);
    addUnitToUnitDictionary(dictonary, side.five);
    addUnitToUnitDictionary(dictonary, side.six);
    addUnitToUnitDictionary(dictonary, side.seven);
    addUnitToUnitDictionary(dictonary, side.eight);
    addUnitToUnitDictionary(dictonary, side.nine);
    addUnitToUnitDictionary(dictonary, side.ten);
    addUnitToUnitDictionary(dictonary, side.eleven);
    addUnitToUnitDictionary(dictonary, side.twelve);
};

const convertBoardToUnitDictionary = (board: Board): Map<string, Unit> => {
    const result: Map<string, Unit>  = new Map();

    // TODO: Figure this out, React tries to call the Grid component with an old (and now free-d Board), so swallow exceptions for now....
    try {

        addSideToUnitDictionary(result, board.black_pieces);
        addSideToUnitDictionary(result, board.white_pieces);
    } catch {}

    return result;
};

const Grid = ({ board, onMove }: GridProps) => {
    const unitDictionary = convertBoardToUnitDictionary(board);
    const [currentSelection, setCurrentSelection] = useState<Unit | null>(null);

    const onOccupiedCellSelection = (unit: Unit) => {
        setCurrentSelection(unit);
    };

    const onEmptyCellSelection = (x: number, y: number) => {

        if (currentSelection) {
            const isValidMove = isValidBoardMove(board, currentSelection, x, y);

            if (isValidMove) {
                setCurrentSelection(null);
                onMove(currentSelection, x, y);
            } else {
                alert('Invalid move!');
            }
        }
    };

    return (<div>
        {[1,2,3,4,5,6,7,8].map((y) => {
            return (<div key={y}>
                {[1,2,3,4,5,6,7,8].map((x) => {
                
                    const getCellContent = () => {
                        const key = `${x},${y}`;
                        const isUnitPresent = unitDictionary.has(key);
            
                        if (isUnitPresent) {
                            const unit = unitDictionary.get(key) as Unit;
            
                            return (
                                <OccupiedCell
                                    key={key}
                                    isSelected={currentSelection?.coordinate.y === unit.coordinate.y && currentSelection?.coordinate.x === unit.coordinate.x}
                                    unit={unit}
                                    onClick={onOccupiedCellSelection}
                                    />
                            );
                        }
            
                        return <EmptyCell key={key} onClick={() => onEmptyCellSelection(x, y)} />;
                    };

                    const cellContent = getCellContent();

                if (x === 1) {
                    return (
                        <>
                            <div style={{ display: 'inline-block' }}>{y}</div>
                            {cellContent}
                        </>
                    )
                }

                return cellContent;
          })}
            </div>);
        })}
    </div>);
};

export default Grid;