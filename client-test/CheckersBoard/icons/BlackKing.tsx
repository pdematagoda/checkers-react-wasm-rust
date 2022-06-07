const BlackKing = () => (
    <svg width={80} height={80}>
        <circle cx={40} cy={40} r={40} fill="black"></circle>
        <circle cx={40} cy={40} r={20} stroke="white" strokeWidth={5} fill="transparent"></circle>
        <path d="M 35 30 V50 M 35 40 L 45 35 M 35 40 L 45 49" strokeWidth={3} stroke="white" />
    </svg>
);

export default BlackKing;