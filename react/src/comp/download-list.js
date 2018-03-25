import React, { Component } from 'react';
import { Table } from 'semantic-ui-react';
import DloadContainer from './dload-container';

export default class DownloadList extends Component {
    render() {
        return <div>
            {this.props.data.map((item, index) => (
                <DloadContainer key={index} container={item} />
            ))}
        </div>
    }
}