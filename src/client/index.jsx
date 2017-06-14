import React from 'react';
import Reflux from 'reflux';
import ReactDOM from 'react-dom';
import MessagePanel from './component/messagePanel.jsx';
import ResponseDialog from './component/responseDialog.jsx';
import EmpholiteStore from './store/empholiteStore.js';
import EmpholiteActions from './action/empholiteActions.js';
import _ from 'lodash';
import {Button, Glyphicon, PageHeader, Panel} from 'react-bootstrap';

class Empholite extends Reflux.Component {
    constructor(props) {
        super(props);
        this.state = {};
        this.store = EmpholiteStore;
    }

    componentDidMount = () => {
       EmpholiteActions.fetchResponses();
    }

    render = () => {
        return (
            <div>
                <PageHeader>
                    <div className='pull-right'>
                        <Button bsStyle='info' bsSize='small' onClick={EmpholiteActions.showResponseDialog}>
                            <Glyphicon glyph='plus' />
                        </Button>
                    </div>
                    Welcome to Empholite <small>Mock any RESTful service</small>
                </PageHeader>
                <MessagePanel message={this.state.message} />
                <ResponseDialog show={this.state.showResponseDialog} />
            </div>
        );
    }
}

ReactDOM.render(<Empholite />, document.querySelector('#empholite'));
