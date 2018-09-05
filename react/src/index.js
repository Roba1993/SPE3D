import global from "./stores/GlobalStore";

import 'semantic-ui-css/semantic.min.css';

import React, { Component } from 'react';
import ReactDOM from 'react-dom';
import { observer } from "mobx-react";
import { HashRouter as Router, Route, Link } from 'react-router-dom';
import { Sidebar, Segment, Button, Menu, Image, Icon, Header } from 'semantic-ui-react';
import Home from './view/home';
import Links from './view/links';
import Config from './view/config';
import Logo from './comp/logo';
import Notify from './comp/Notify';


@observer
class App extends Component {

    deleteLink(e) {
        e.preventDefault();
        global.dload.removeDload(global.ui.selected);
    }

    startDownload(e) {
        e.preventDefault();
        global.dload.startDload(global.ui.selected);
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
                        <Link to="/config" style={styleButton}><Icon name='settings' size='large' /></Link>
                    </Menu.Item>
                </Sidebar>
                <Sidebar.Pusher>
                    <Segment basic style={styleSegment}>
                        <Route exact path="/" render={() => <Home global={global} />} />
                        <Route path="/links" render={() => <Links global={global} />} />
                        <Route path="/config" render={() => <Config global={global} />} />
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