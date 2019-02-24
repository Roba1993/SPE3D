import global from "./stores/GlobalStore";

import 'semantic-ui-css/semantic.min.css';

import React, { Component } from 'react';
import ReactDOM from 'react-dom';
import { observer } from "mobx-react";
import { BrowserRouter as Router, Route, Link } from 'react-router-dom';
import { Sidebar, Segment, Button, Menu, Image, Icon, Header } from 'semantic-ui-react';
import Home from './view/home';
import Links from './view/links';
import Config from './view/config';
import Logo from './comp/logo';
import Notify from './comp/Notify';
import AddAccount from './comp/add-account';

@observer
class App extends Component {

    deleteLink(e) {
        e.preventDefault();
        global.dload.con.removeDloadById(global.ui.selected);
    }

    startDownload(e) {
        e.preventDefault();
        global.dload.con.startDloadById(global.ui.selected);
    }

    deleteAccount(e) {
        e.preventDefault();
        global.config.con.removeAccount(global.ui.accountSelected);
        global.ui.accountSelected = false;
    }

    getMenu() {
        if (this.props.global.ui == undefined) {
            return <Menu.Item name='actions' style={styleButtons} active={false}>
                <Link to="/links" style={styleButton}><Icon name='plus' size='large' /></Link>
            </Menu.Item>
        }

        if (this.props.global.ui.path == '/') {
            return <Menu.Item name='actions' style={styleButtons} active={false}>
                <Link to="/links" style={styleButton}><Icon name='plus' size='large' /></Link>
                <Icon name='trash' size='large' color={this.props.global.ui.selected ? 'green' : 'grey'} onClick={(e) => { this.deleteLink(e) }} />
                <Icon name='arrow down' size='large' color={this.props.global.ui.selected ? 'green' : 'grey'} onClick={(e) => { this.startDownload(e) }} />
            </Menu.Item>
        }
        else if (this.props.global.ui.path == '/config' && this.props.global.ui.configTab == 'accounts') {
            return <Menu.Item name='actions' style={styleButtons} active={false}>
                <Icon.Group size='large' onClick={(e) => { global.ui.modalAddAccount = true }}><Icon name='key' /><Icon corner name='plus' /></Icon.Group>
                <Icon name='trash' size='large' color={this.props.global.ui.accountSelected ? 'green' : 'grey'} onClick={(e) => { this.deleteAccount(e) }} />
            </Menu.Item>
        }
    }

    render() {
        return <Router>
            <Sidebar.Pushable as={Page}>
                <Sidebar as={Menu} animation='push' direction='top' visible={true} size='large' borderless>
                    <Menu.Item name='name'>
                        <Link to="/" style={styleMenu}><Logo height='25px' /></Link>
                    </Menu.Item>
                    {this.getMenu()}
                    <Menu.Item name='links' position='right' style={styleLinks}>
                        <Link to="/" style={styleButton}><Icon name='download' size='large' /></Link>
                        <Link to="/links" style={styleButton}><Icon name='plus' size='large' /></Link>
                        <Link to="/config" style={styleButton}><Icon name='settings' size='large' /></Link>
                    </Menu.Item>
                </Sidebar>
                <Sidebar.Pusher>
                    <Segment basic position='right' style={styleSegment}>
                        <Route exact path="/" render={(r) => { global.ui.path = r.location.pathname; return <Home global={global} /> }} />
                        <Route path="/links" render={(r) => { global.ui.path = r.location.pathname; return <Links global={global} /> }} />
                        <Route path="/config" render={(r) => { global.ui.path = r.location.pathname; return <Config location={location} global={global} /> }} />
                        <AddAccount global={global}></AddAccount>
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
    marginLeft: 'auto',
    marginRight: 'auto'
}

const styleSegment = {
    color: '#00ca34',
    marginTop: '20px'
}

const styleButton = {
    color: '#00ca34',
}

const styleLinks = {
    color: '#00ca34',
    marginRight: '10px'
}

ReactDOM.render(<App global={global} />, document.getElementById('app'))