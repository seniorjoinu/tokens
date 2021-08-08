import {Actor, HttpAgent, Identity} from "@dfinity/agent";
import fetch from 'node-fetch';
import {exec} from 'child_process';
import {expect} from "chai";

import MembershipTokenClient from 'dfx-type/membership-token/membership-token';
import CurrencyTokenClient from 'dfx-type/currency-token/currency-token';


export interface IMembershipTokenSetup {
    agent: HttpAgent;
    membershipTokenClient: MembershipTokenClient;
}

export async function makeMembershipTokenSetup(identity: Identity): Promise<IMembershipTokenSetup> {
    const agent = new HttpAgent({
        host: 'http://localhost:8000/',
        // @ts-ignore
        fetch,
        identity
    });

    const {
        canisterId: membershipTokenCanisterId,
        idlFactory: membershipTokenIdl
    } = await import('dfx/membership-token/membership-token');

    const membershipTokenClient: MembershipTokenClient = Actor.createActor(membershipTokenIdl, {
        agent,
        canisterId: membershipTokenCanisterId
    });

    return {
        agent,
        membershipTokenClient
    };
}

export interface ICurrencyTokenSetup {
    agent: HttpAgent;
    currencyTokenClient: CurrencyTokenClient;
}

export async function makeCurrencyTokenSetup(identity: Identity): Promise<ICurrencyTokenSetup> {
    const agent = new HttpAgent({
        host: 'http://localhost:8000/',
        // @ts-ignore
        fetch,
        identity
    });

    const {
        canisterId: currencyTokenCanisterId,
        idlFactory: currencyTokenIdl
    } = await import("dfx/currency-token/currency-token");

    const currencyTokenClient: CurrencyTokenClient = Actor.createActor(currencyTokenIdl, {
        agent,
        canisterId: currencyTokenCanisterId
    });

    return {
        agent,
        currencyTokenClient
    }
}

export function getTimeNano(): bigint {
    return BigInt(new Date().getTime() * 1000_000)
}

export function getHoursNano(h: number): bigint {
    return BigInt(1000_000_000 * 60 * 60 * h);
}

export function getSecsNano(s: number): bigint {
    return BigInt(1000000000 * s);
}

export function getMinsNano(m: number): bigint {
    return BigInt(1000000000 * 60 * m);
}

export function bnToBuf(bn: bigint) {
    let hex = BigInt(bn).toString(16);
    if (hex.length % 2) {
        hex = '0' + hex;
    }

    let len = hex.length / 2;
    let u8 = new Uint8Array(len);

    let i = 0;
    let j = 0;
    while (i < len) {
        u8[i] = parseInt(hex.slice(j, j + 2), 16);
        i += 1;
        j += 2;
    }

    return u8;
}

export function bufToBn(arr: Uint8Array) {
    let hex: string[] = [];

    arr.forEach((i) => {
        let h = i.toString(16);
        if (h.length % 2) {
            h = '0' + h;
        }
        hex.push(h);
    });

    return BigInt('0x' + hex.join(''));
}

export function objectEquals(x: any, y: any) {
    if (x === y) return true;
    // if both x and y are null or undefined and exactly the same

    if (!(x instanceof Object) || !(y instanceof Object)) return false;
    // if they are not strictly equal, they both need to be Objects

    if (x.constructor !== y.constructor) return false;
    // they must have the exact same prototype chain, the closest we can do is
    // test there constructor.

    for (var p in x) {
        if (!x.hasOwnProperty(p)) continue;
        // other properties were tested using x.constructor === y.constructor

        if (!y.hasOwnProperty(p)) return false;
        // allows to compare x[ p ] and y[ p ] when set to undefined

        if (x[p] === y[p]) continue;
        // if they have the same strict value or identity then they are equal

        if (typeof (x[p]) !== "object") return false;
        // Numbers, Strings, Functions, Booleans must be strictly equal

        if (!objectEquals(x[p], y[p])) return false;
        // Objects and Arrays must be tested recursively
    }

    for (p in y)
        if (y.hasOwnProperty(p) && !x.hasOwnProperty(p))
            return false;
    // allows x[ p ] to be set to undefined

    return true;
}

export async function execAsync(command: string) {
    return new Promise((res, rej) => {
        exec(command, (err, stderr, stdout) => {
            if (err) {
                rej(err);
            } else if (stderr) {
                rej(stderr);
            } else if (stdout) {
                res(stdout);
            }
            return;
        })
    })
}

export const expectThrowsAsync = async (method: Promise<any>, errorMessage?: string) => {
    let error = null
    try {
        await method
    } catch (err) {
        error = err
    }

    expect(error).to.be.an('Error', errorMessage);
}

export async function delay(ms: number) {
    return new Promise(resolve => setTimeout(resolve, ms));
}