import React, { Component } from 'react'
import { observer } from "mobx-react"
import { Form, Segment } from 'semantic-ui-react'

@observer
export default class ConfigServer extends Component {
    handleChange = (e, { name, value }) => {
        this.props.global.config.server[name] = value;
    }

    handleSubmit = () => {
        // send update request
    }

    render() {
        if( this.props.global.config.server == undefined) {
            return(<Segment>Config is Loading...</Segment>);
        }
        const server = this.props.global.config.server;

        return (
            <Segment>
                <Form onSubmit={this.handleSubmit}>
                    <Form.Input label='Server IP' name='ip' value={server.ip} onChange={this.handleChange} />
                    <Form.Input label='Webserver Port' name='webserver_port' value={server.webserver_port} onChange={this.handleChange} />
                    <Form.Input label='Websocket Port' name='websocket_port' value={server.websocket_port} onChange={this.handleChange} />
                    <Form.Button content='Save' />
                </Form>
            </Segment>
        )
    }
}