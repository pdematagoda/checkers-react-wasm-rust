import { Colour, Unit, UnitType } from "wasm-ai-thingo";

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
    const unitSuffix = unit.unit_type === UnitType.Pawn ? '' : 'K';

    return (<div style={{ minWidth: 50, minHeight: 50, background: isSelected ? 'yellow' : 'white', display: 'inline-block', border: '1px lightgrey solid', textAlign: 'center' }} onClick={() => onClick(unit)}>
        <span style={{ fontSize: 18 }}>{unit.colour === Colour.Black ? `Black ${unitSuffix}` : `White ${unitSuffix}`}</span>
    </div>);
};

export default OccupiedCell;