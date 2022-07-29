import React from "react";

class Secret extends React.Component {
    constructor(props) {
        super(props);
        this.state = ({ 
            secret: props.word,
            value: "_ _ _ _ _ _ _"
        });
    
    }
    render(){
        return (
            <div>
                <h2>{this.state.value}</h2>
            </div>
        );
    }
}

export default Secret; 