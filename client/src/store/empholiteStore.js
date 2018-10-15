import Reflux from 'reflux';
import EmpholiteActions from '../action/empholiteActions.js';
import SuperAgent from 'superagent';
import Bluebird from 'bluebird';
import SuperAgentPromise from 'superagent-promise';
const superagent = SuperAgentPromise(SuperAgent, Bluebird);

// `window.location.origin` doesn't exist in some browsers, so fill it in:
if (!window.location.origin) {
    window.location.origin = `${window.location.protocol}//${window.location.hostname}`;

    if (window.location.port) {
        window.location.origin = `${window.location.origin}:${window.location.port}`;
    }
}

const errorMessages = {
    404: 'Not found.'
};

function errorHandler(message, store) {
    return error => {
        store.trigger({
            message: {
                context: 'danger',
                text: errorMessages[error.status] ? errorMessages[error.status] : message
            }
        });
    };
}

function superagentSendXhrHeader(request) {
    request.set('X-Requested-With', 'XMLHttpRequest');

    return request;
}

function getAndUnwrap(endpoint) {
    return superagent.get(endpoint)
        .use(superagentSendXhrHeader)
        .then(result => {
            const data = JSON.parse(result.text);
            if (data.success) {
                return data;
            } else {
                if (data.error === undefined) {
                    throw new Error(`Malformed response, ${result.text}`);
                }
                throw data.error;
            }
        });
}

function postAndUnwrap(endpoint, data) {
    return superagent.post(endpoint)
        .use(superagentSendXhrHeader)
        .send(data)
        .then(result => {
            const data = JSON.parse(result.text);
            if (data.success) {
                return data;
            } else {
                if (data.error === undefined) {
                    throw new Error(`Malformed response, ${result.text}`);
                }
                throw data.error;
            }
        });
}

export default class EmpholiteStore extends Reflux.Store {
    constructor() {
        super();
        this.state = {
            message: {},
            showResponseDialog: false
        };
        this.listenToMany(EmpholiteActions);
    }

    onClearMessage() {
        this.setState({message: {}});
    }

    onFetchResponses() {
        this.trigger({message: {}});
        getAndUnwrap(`${window.location.origin}/ajax/list`)
            .then(data => {
                this.setState(data);
            })
            .catch(errorHandler('Problem fetching responses.', this));
    }

    onShowResponseDialog() {
        this.trigger({message: {}});
        this.trigger({showResponseDialog: true});
    }

    onHideResponseDialog() {
        this.trigger({showResponseDialog: false});
    }
}
