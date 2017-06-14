import React from 'react';
import Reflux from 'reflux';
import ReactDOM from 'react-dom';
import MessagePanel from './component/messagePanel.jsx';
import EmpholiteStore from './store/empholiteStore.js';
import EmpholiteActions from './action/empholiteActions.js';
import _ from 'lodash';
import {PageHeader, Panel} from 'react-bootstrap';

class Empholite extends Reflux.Component {
    constructor(props) {
        super(props);
        this.state = {};
        this.store = EmpholiteStore;
    }

    componentDidMount = () => {
        PortcullisActions.fetchSession();
    }

    render = () => {
        return (
            <div>
                <PageHeader>
                    Welcome to Empholite <small>Mock any RESTful service</small>
                </PageHeader>
                <MessagePanel message={this.state.message} />
            </div>
        );
    }
}

ReactDOM.render(<Empholite />, document.querySelector('#empholite'));
