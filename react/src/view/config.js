import React, { Component } from 'react'
import { Grid, Menu, Segment, Icon, Form } from 'semantic-ui-react'
import ConfigServer from '../comp/config-server'
import ConfigAccounts from '../comp/config-accounts'

export default class Config extends Component {
    state = { activeItem: 'server' }

    handleItemClick = (e, { name }) => {
        this.setState({ activeItem: name });
        // with each menu change we load the actual config data
        this.props.global.config.fetchConfig();
    }

    render() {
        const { activeItem } = this.state;
        let content;

        switch (activeItem) {
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
                        <Menu.Item name='server' active={activeItem === 'server'} onClick={this.handleItemClick} >
                            <Icon name='server' />
                            Server
                        </Menu.Item>
                        <Menu.Item name='accounts' active={activeItem === 'accounts'} onClick={this.handleItemClick}  >
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