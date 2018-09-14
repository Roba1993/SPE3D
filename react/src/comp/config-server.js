import React, { Component } from 'react'
import { observer } from "mobx-react"
import { Form, Segment } from 'semantic-ui-react'

@observer
export default class ConfigServer extends Component {
    handleChange = (e, { name, value }) => {
        if(name == 'port') {
            this.props.global.config.server.setPort(value);
            return;
        }

        this.props.global.config.server[name] = value;
    }

    handleSubmit = () => {
        this.props.global.config.updateServer();
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
                    <Form.Input label='Port' name='port' value={server.port} onChange={this.handleChange} />
                    <Form.Button content='Save' />
                </Form>
            </Segment>
        )
    }
}