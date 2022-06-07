import { Colour, Unit, UnitType } from "wasm-ai-thingo";
import getCellBackgroundColourForCoordinate from "./getCellBackgroundColour";

interface OccupiedCellProps {
    unit: Unit;
    onClick: (unit: Unit) => void;
    isSelected: boolean;
}

const OccupiedCell = ({
    unit,
    onClick,
    isSelected
}: OccupiedCellProps) => {
    const {
        colour,
        coordinate: {
            x,
            y
        },
        unit_type
    } = unit;
    const unitSuffix = unit_type === UnitType.Pawn ? '' : 'K';

    return (
    <div
        title={`${x}x , ${y}y`}
        style={{ background: isSelected ? 'yellow' : getCellBackgroundColourForCoordinate(x, y), height: '100%', width: '100%', border: '1px lightgrey solid', textAlign: 'center', }}
        onClick={() => onClick(unit)}
        >
        <span style={{ fontSize: 18 }}>{colour === Colour.Black ? `Black ${unitSuffix}` : `White ${unitSuffix}`}</span>
    </div>
    );
};

export default OccupiedCell;