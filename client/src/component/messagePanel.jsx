import React from 'react';
import PropTypes from 'prop-types';
import EmpholiteActions from '../action/empholiteActions.js';
import {Alert} from 'react-bootstrap';

export default class MessagePanel extends React.Component {
    static propTypes = {
        message: PropTypes.shape({
            context: PropTypes.oneOf(['success', 'info', 'warning', 'danger']),
            text: PropTypes.string
        })
    }

    handleClose() {
        PortcullisActions.clearMessage();
    }

    render = () => {
        if (!this.props.message.text) {
            return null;
        }
        return (
            <Alert bsStyle={this.props.message.context} onDismiss={this.handleClose}>
                {this.props.message.text}
            </Alert>
        );
    }
}
