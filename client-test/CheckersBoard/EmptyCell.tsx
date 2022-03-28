interface EmptyCellProps {
    onClick: () => void;
}

const EmptyCell = ({ onClick } : EmptyCellProps) => {
    return (<div style={{ minWidth: 50, minHeight: 50, background: 'white', display: 'inline-block', border: '1px lightgrey solid' }} onClick={onClick}>
        -
    </div>);
};

export default EmptyCell;