import React, { Component } from 'react'
import { observer } from "mobx-react"
import { Button, Image, List, Table, Segment } from 'semantic-ui-react'

@observer
export default class ConfigServer extends Component {
    handleChange = (e, { name, value }) => {
        this.props.global.config.server[name] = value;
    }

    select = (id) => {
        if (this.props.global.ui.accountSelected == id) {
            this.props.global.ui.accountSelected = false;
        }
        else {
            this.props.global.ui.accountSelected = id;
        }

        this.forceUpdate()
    }

    isSelected = (id) => {
        if (this.props.global.ui.accountSelected == id) {
            return true
        }
        else {
            return false
        }
    }

    render() {
        return (
            <Segment>
                <Table basic='very' selectable>
                    <Table.Header>
                        <Table.Row>
                            <Table.HeaderCell>Hoster</Table.HeaderCell>
                            <Table.HeaderCell>ID</Table.HeaderCell>
                            <Table.HeaderCell>User</Table.HeaderCell>
                            <Table.HeaderCell>Password</Table.HeaderCell>
                            <Table.HeaderCell>Last Checked</Table.HeaderCell>
                            <Table.HeaderCell>Status</Table.HeaderCell>
                        </Table.Row>
                    </Table.Header>
                    <Table.Body>
                        {this.props.global.config.accounts.map((item, index) => (
                            <Table.Row key={index} positive={this.isSelected(item.id)} onClick={(e) => { this.select(item.id) }}>
                                <Table.Cell>{this.props.global.config.getHosterImage(item.hoster)} {item.hoster}</Table.Cell>
                                <Table.Cell>{item.id}</Table.Cell>
                                <Table.Cell>{item.username}</Table.Cell>
                                <Table.Cell>{item.password}</Table.Cell>
                                <Table.Cell>{item.checked}</Table.Cell>
                                <Table.Cell>{item.status}</Table.Cell>
                            </Table.Row>
                        ))}
                    </Table.Body>
                </Table>
            </Segment>
        )
    }
}