import React, { Component } from 'react'
import { observer } from "mobx-react"
import { Grid, Menu, Segment, Icon, Form } from 'semantic-ui-react'
import ConfigServer from '../comp/config-server'
import ConfigAccounts from '../comp/config-accounts'

@observer
export default class Config extends Component {
    handleItemClick = (e, { name }) => {
        this.props.global.ui.configTab = name;
        // with each menu change we load the actual config data
        this.props.global.config.fetchConfig();
    }

    render() {
        const { configTab } = this.props.global.ui;
        let content;

        switch (configTab) {
            case 'accounts':
                content = <ConfigAccounts global={this.props.global} />
                break;
            default:
                content = <ConfigServer global={this.props.global} />
                break;
        }

        return (
            <Grid>
                <Grid.Column width={4}>
                    <Menu fluid vertical tabular size='large'>
                        <Menu.Item name='server' active={configTab === 'server'} onClick={this.handleItemClick} >
                            <Icon name='server' />
                            Server
                        </Menu.Item>
                        <Menu.Item name='accounts' active={configTab === 'accounts'} onClick={this.handleItemClick}  >
                            <Icon name='key' />
                            Accounts
                        </Menu.Item>
                    </Menu>
                </Grid.Column>

                <Grid.Column stretched width={12}>
                    {content}
                </Grid.Column>
            </Grid>
        )
    }
}