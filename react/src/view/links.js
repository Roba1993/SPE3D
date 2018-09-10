import React, { Component } from 'react';
import { observer } from "mobx-react";
import { Grid } from 'semantic-ui-react'
import AddLinks from '../comp/add-links';
import AddFile from '../comp/add-file';

@observer
export default class Links extends Component {
    render() {
         return <Grid columns={2} divided>
            <Grid.Row>
                <Grid.Column>
                    <AddLinks global={this.props.global} />
                </Grid.Column>
                <Grid.Column>
                    <AddFile global={this.props.global} />
                </Grid.Column>
            </Grid.Row>
        </Grid>
    }
}