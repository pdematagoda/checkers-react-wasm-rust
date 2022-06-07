const getCellBackgroundColourForCoordinate = (x: number, y: number) => {
    return (x + y) % 2 === 1 ? 'white' : 'lightgrey';
};

export default getCellBackgroundColourForCoordinate;