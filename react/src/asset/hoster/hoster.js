import React, { Component } from 'react'
import { Image } from 'semantic-ui-react';
import filer from './filer.png';
import other from './other.png';
import shareonline from './shareonline.png';


export default class HosterImages extends Component {
    static get(hoster) {
        switch(hoster) {
            case "Filer":
                return HosterImages.filer();
            case "Share-Online":
                return HosterImages.shareonline();
            default:
                return HosterImages.other();
        }
    }

    static other() {
        return <Image src={other} avatar />;
    }

    static filer() {
        return <Image src={filer} avatar />;
    }

    static shareonline() {
        return <Image src={shareonline} avatar />;
    }
}