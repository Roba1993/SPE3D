import 'semantic-ui-css/semantic.min.css';

import React, { Component } from 'react';
import ReactDOM from 'react-dom';
import { observer } from "mobx-react";
import { HashRouter as Router, Route, Link } from 'react-router-dom';
import { Sidebar, Segment, Button, Menu, Image, Icon, Header } from 'semantic-ui-react';
import Home from './view/home';
import Links from './view/links';
import Logo from './comp/logo';
import Notify from './comp/Notify';
import global from "./stores/GlobalStore";

@observer
class App extends Component {
    state = {
        dloads: [],
    };

    componentDidMount() {
        this.loadLinks();
        this.ws();
    }

    loadLinks() {
        fetch(`http://` + window.location.hostname + `:8000/api/downloads`)
            .then(res => {
                if (res.status != 200) {
                    this.props.global.notify.createErrorMsg("Download list not avialable", "The server was not able to provide the download list");
                }

                return res.json()
            })
            .then(dloads => this.setState({ dloads: dloads }))
            .catch(error => {
                this.props.global.notify.createErrorMsg("Connection to server failed", "Can't get the download list from server");
            });
    }

    ws() {
        var websocket = new WebSocket('ws://' + window.location.hostname + ':8001');
        var that = this;
        websocket.onmessage = function (evt) {
            var obj = JSON.parse(evt.data);
            //console.log(obj);
            that.setState({ dloads: obj });
        };
    }

    deleteLink(e) {
        e.preventDefault();

        if (!this.props.global.ui.selected) {
            return;
        }

        var id = this.props.global.ui.selected;
        fetch("http://" + window.location.hostname + ":8000/api/delete-link/" + id,
            {
                method: "POST",
                headers: {
                    'Accept': 'application/json, text/plain, */*',
                    'Content-Type': 'application/json'
                },
                //body: JSON.stringify(this.state)
            })
            .then(res => {
                if (res.status != 200) {
                    this.props.global.notify.createErrorMsg("Deletion failed", "The server was not able to remove the link");
                }
            })
    }

    startDownload(e) {
        e.preventDefault();

        if (!this.props.global.ui.selected) {
            return;
        }

        var id = this.props.global.ui.selected;
        fetch("http://" + window.location.hostname + ":8000/api/start-download/" + id,
            {
                method: "POST"
            })
            .then(res => { 
                if (res.status != 200) {
                    this.props.global.notify.createErrorMsg("Download not started", "The server was not able to start the download");
                }
            })
    }

    render() {
        return <Router>
            <Sidebar.Pushable as={Page}>
                <Sidebar as={Menu} animation='push' direction='top' visible={true} size='large' borderless>
                    <Menu.Item name='name'>
                        <Link to="/" style={styleMenu}><Logo height='25px' width='100%' /></Link>
                    </Menu.Item>
                    <Menu.Item name='links' style={styleButtons}>
                        <Link to="/links" style={styleButton}><Icon name='plus' size='large' /></Link>
                        <Icon name='trash' size='large' color={this.props.global.ui.selected ? 'green' : 'grey'} onClick={(e) => { this.deleteLink(e) }} />
                        <Icon name='arrow down' size='large' color={this.props.global.ui.selected ? 'green' : 'grey'} onClick={(e) => { this.startDownload(e) }} />
                    </Menu.Item>
                </Sidebar>
                <Sidebar.Pusher>
                    <Segment basic style={styleSegment}>
                        <Route exact path="/" render={() => <Home global={global} dloads={this.state.dloads} />} />
                        <Route path="/links" render={() => <Links global={global} />} />
                    </Segment>
                </Sidebar.Pusher>
                <Notify global={global} />
            </Sidebar.Pushable>
        </Router>
    }
}

class Page extends Component {
    render() {
        return <div>
            {this.props.children}
        </div>
    }
}

const styleMenu = {
    color: '#00ca34',
    marginLeft: '10px'
}

const styleButtons = {
    color: '#00ca34',
    marginLeft: '200px'
}

const styleSegment = {
    color: '#00ca34',
    marginTop: '20px'
}

const styleButton = {
    color: '#00ca34',
}


ReactDOM.render(<App global={global} />, document.getElementById('app'))