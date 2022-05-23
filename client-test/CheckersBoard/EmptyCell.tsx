interface EmptyCellProps {
    onClick: () => void;
    background: string;
}

const EmptyCell = ({ onClick, background } : EmptyCellProps) => {
    return (<div style={{ minWidth: 50, minHeight: 50, background: background, display: 'inline-block', border: '1px lightgrey solid' }} onClick={onClick}>
        -
    </div>);
};

export default EmptyCell;