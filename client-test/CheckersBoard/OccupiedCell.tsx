import { Colour, Unit, UnitType } from "wasm-ai-thingo";
import getCellBackgroundColourForCoordinate from "./getCellBackgroundColour";
import { BlackKing, BlackPawn, WhiteKing, WhitePawn } from "./icons";

interface OccupiedCellProps {
    unit: Unit;
    onClick: (unit: Unit) => void;
    isSelected: boolean;
}

const getUnitIcon = (colour: Colour, unitType: UnitType) => {
    if (unitType === UnitType.King) {
        if (colour === Colour.Black) {
            return BlackKing;
        }

        return WhiteKing;
    }

    return colour === Colour.White ? WhitePawn : BlackPawn;
};

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
        unitType
    } = unit;
    const UnitIcon = getUnitIcon(colour, unitType);

    return (
    <div
        title={`${x}x , ${y}y`}
        style={{ background: isSelected ? '#CC9933' : getCellBackgroundColourForCoordinate(x, y), height: '100%', width: '100%', border: '1px lightgrey solid', textAlign: 'center', }}
        onClick={() => onClick(unit)}
        >
        <div style={{ paddingTop: 10 }}>
            <UnitIcon />
        </div>
    </div>
    );
};

export default OccupiedCell;