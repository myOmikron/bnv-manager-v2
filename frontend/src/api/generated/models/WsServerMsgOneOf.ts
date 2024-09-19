/* tslint:disable */
/* eslint-disable */
/**
 * Frontend
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v0.0.0
 * 
 *
 * NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).
 * https://openapi-generator.tech
 * Do not edit the class manually.
 */

import { mapValues } from '../runtime';
import type { WebconfUpdateResult } from './WebconfUpdateResult';
import {
    WebconfUpdateResultFromJSON,
    WebconfUpdateResultFromJSONTyped,
    WebconfUpdateResultToJSON,
} from './WebconfUpdateResult';

/**
 * Deployment state has updated
 * @export
 * @interface WsServerMsgOneOf
 */
export interface WsServerMsgOneOf {
    /**
     * 
     * @type {WebconfUpdateResult}
     * @memberof WsServerMsgOneOf
     */
    state: WebconfUpdateResult;
    /**
     * The task uuid
     * @type {string}
     * @memberof WsServerMsgOneOf
     */
    task: string;
    /**
     * 
     * @type {string}
     * @memberof WsServerMsgOneOf
     */
    type: WsServerMsgOneOfTypeEnum;
}


/**
 * @export
 */
export const WsServerMsgOneOfTypeEnum = {
    DeployUpdate: 'DeployUpdate'
} as const;
export type WsServerMsgOneOfTypeEnum = typeof WsServerMsgOneOfTypeEnum[keyof typeof WsServerMsgOneOfTypeEnum];


/**
 * Check if a given object implements the WsServerMsgOneOf interface.
 */
export function instanceOfWsServerMsgOneOf(value: object): value is WsServerMsgOneOf {
    if (!('state' in value) || value['state'] === undefined) return false;
    if (!('task' in value) || value['task'] === undefined) return false;
    if (!('type' in value) || value['type'] === undefined) return false;
    return true;
}

export function WsServerMsgOneOfFromJSON(json: any): WsServerMsgOneOf {
    return WsServerMsgOneOfFromJSONTyped(json, false);
}

export function WsServerMsgOneOfFromJSONTyped(json: any, ignoreDiscriminator: boolean): WsServerMsgOneOf {
    if (json == null) {
        return json;
    }
    return {
        
        'state': WebconfUpdateResultFromJSON(json['state']),
        'task': json['task'],
        'type': json['type'],
    };
}

export function WsServerMsgOneOfToJSON(value?: WsServerMsgOneOf | null): any {
    if (value == null) {
        return value;
    }
    return {
        
        'state': WebconfUpdateResultToJSON(value['state']),
        'task': value['task'],
        'type': value['type'],
    };
}

