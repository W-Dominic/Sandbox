import React from "react";

class Guess extends React.Component {
    constructor(props){
        super(props);
        this.state = ({guess: props.parent});

        //this.handleSubmit = this.handleSubmit.bind(this);
        //this.handleChange = this.handleChange.bind(this);
    }
    render(){
        return (
            <div>
                <form onSubmit={this.props.handleFormSubmit}>
                    <input type="text" placeholder="enter letter" name="inputBox" onChange={this.props.onFormChange}/>
                    <button type="submit" className="btn btn-primary">Submit</button>
                </form>
            </div>
        );
    }
}

export default Guess;