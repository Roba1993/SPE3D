import React, { Component } from 'react';
import { Grid } from 'semantic-ui-react'
import AddLinks from '../comp/add-links';
import AddFile from '../comp/add-file';

export default class Links extends Component {
    render() {
        return <Grid columns={2} divided>
            <Grid.Row>
                <Grid.Column>
                    <AddLinks />
                </Grid.Column>
                <Grid.Column>
                    <AddFile />
                </Grid.Column>
            </Grid.Row>

        </Grid>
    }
}