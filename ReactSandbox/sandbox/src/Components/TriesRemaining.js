import React from "react";

class TriesRemaining extends React.Component {
    constructor(props) {
        super(props);
        this.state = ({tries : props.parent}); 
    }
    render() {
        return (
            <div>
                <h1>Tries Remaining: {this.state.tries}</h1>
            </div>
        );
    }
}

export default TriesRemaining;  