import React, { Component } from 'react';
import { observer } from "mobx-react";
import { Input, Icon, Button, Header, Message } from 'semantic-ui-react'

@observer
export default class Notify extends Component {

    render() {
        let { notify } = this.props.global;

        const msgList = notify.messages.map(msg => (
            <Message 
                positive={msg.type=="ok"?true:false}
                negative={msg.type=="error"?true:false}
                icon={msg.icon}
                style={messageStyle} 
                key={msg.id} 
                onDismiss={() => notify.deleteMsg(msg.id)}
                header={msg.name}
                content={msg.description}
            />
        ))

        return <div style={msgBoxStyle}>
            {msgList}
        </div>
    }
}

const messageStyle = {
    position: 'relative',
};

const msgBoxStyle = {
    position: 'fixed',
    bottom: '5px',
    right: '5px',
}