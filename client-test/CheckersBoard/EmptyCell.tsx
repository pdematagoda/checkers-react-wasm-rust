import getCellBackgroundColourForCoordinate from "./getCellBackgroundColour";

interface EmptyCellProps {
    onClick: () => void;
    background?: string;
    x: number;
    y: number;
}

const EmptyCell = ({ onClick, background, x, y } : EmptyCellProps) => {
    return (<div style={{ background: background ?? getCellBackgroundColourForCoordinate(x, y), height: '100%', width: '100%', border: '1px lightgrey solid' }} title={`${x}x , ${y}y`} onClick={onClick}>
    </div>);
};

export default EmptyCell;