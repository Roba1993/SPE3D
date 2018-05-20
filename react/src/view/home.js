import React, { Component } from 'react';
import { observer } from "mobx-react";
import DownloadList from './../comp/download-list';

@observer
export default class Home extends Component {
    render() {
        return <div>
            <DownloadList global={this.props.global} data={this.props.dloads} />
        </div>
    }
}