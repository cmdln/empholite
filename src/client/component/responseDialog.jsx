import React from 'react';
import PropTypes from 'prop-types';
import EmpholiteActions from '../action/empholiteActions.js';
import {Button, Modal} from 'react-bootstrap';
import _ from 'lodash';

ResponseDialog.propTypes = {
    show: PropTypes.bool
}

function delayedHide() {
    _.delay(EmpholiteActions.hideResponseDialog, 100);
}

function ResponseDialog(props) {
    return (
        <Modal show={props.show} onHide={delayedHide}>
            <Modal.Header closeButton={true}>
                Create a Response
            </Modal.Header>
            <Modal.Body>
                TextArea for JSON
            </Modal.Body>
            <Modal.Footer>
                <Button>Create</Button>
            </Modal.Footer>
        </Modal>
    );
}

export default ResponseDialog;
