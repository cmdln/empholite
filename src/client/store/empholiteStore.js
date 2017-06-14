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
    503: 'Tower or its dependencies appear to be unavailable at the moment.',
    401: 'Forbidden, your SSO session may have expired. Try refreshing your browser.'
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

function fetchSession(store) {
    store.trigger({message: {}});
    getAndUnwrap(`${window.location.origin}/ajax/check`)
        .then(data => {
            store.setState(data);
            if (!data.displayLogin && !data.hasSession) {
                store.setState({message: {
                    context: 'warning',
                    text: 'Your session has logged out or expired, try restarting your SSO attempt.'
                }});
            }
        })
        .catch(errorHandler('Problem checking validity.', store));
}

export default class EmpholiteStore extends Reflux.Store {
    constructor() {
        super();
        this.state = {
            message: {},
            userDetails: {},
            hasSession: false,
            displayLogin: false,
            signOutUrls: [],
            profileInRequest: false,
            profileInSession: false
        };
        this.listenToMany(EmpholiteActions);
    }

    onClearMessage() {
        this.setState({message: {}});
    }

    onFetchSession() {
        fetchSession(this);
    }

    onUpdateUserDetail(name, value) {
        let userDetails = Object.assign({}, this.state.userDetails);
        userDetails[name] = value;
        this.setState({userDetails});
    }

    onLogin() {
        EmpholiteActions.clearMessage();
        postAndUnwrap(`${window.location.origin}/ajax/login`, this.state.userDetails)
            .then(({redirectUri}) => {
                window.location = redirectUri;
            })
            .catch(errorHandler('Problem logging in.', this));
    }
}
