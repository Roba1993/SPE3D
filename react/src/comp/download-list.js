import React, { Component } from 'react';
import { observer } from "mobx-react";
import { Table, Grid, Header } from 'semantic-ui-react';
import DloadContainer from './dload-container';

@observer
export default class DownloadList extends Component {
    render() {
        return <div>
            <Grid columns='equal'>
                <Grid.Column computer={1} mobile={4} style={styleCenter}>
                    <Header as='h3' style={styleText}>File</Header>
                </Grid.Column>
                <Grid.Column computer={5} mobile={12} centered='true' style={styleCenter}>
                    <Header as='h3' style={styleText}>Name</Header>
                </Grid.Column>
                <Grid.Column computer={3} style={styleCenter}>
                    <Header as='h3' style={styleText}>Speed</Header>
                </Grid.Column>
                <Grid.Column computer={2} style={styleCenter}>
                    <Header as='h3' style={styleText}>Size</Header>
                </Grid.Column>
                <Grid.Column computer={2} style={styleCenter}>
                    <Header as='h3' style={styleText}>Time</Header>
                </Grid.Column>
                <Grid.Column computer={2} style={styleCenter}>
                    <Header as='h3' style={styleText}>Downloaded</Header>
                </Grid.Column>
            </Grid>
            {this.props.global.dload.dloads.map((item, index) => (
                <DloadContainer key={item.id} container={item} global={this.props.global} />
            ))}
        </div>
    }
}

const styleCenter = {
    display: 'flex',
    alignItems: 'center',
    justifyContent: 'center'
}

const styleText = {
    color: '#acabab',
    fontWeight: '100',
}