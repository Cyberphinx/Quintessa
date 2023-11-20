import { writable } from 'svelte/store';

export class Clock {
    time: number;
    private _store: any;
    constructor() {
        this.time = Date.now();
        this._store = writable(this);
    }

    tick() {
        this.time = Date.now();
        this._store.set(this);
        // 		console.log(`tick(): ${this.time}`);
    }

    subscribe(subscriber: any) {
        return this._store.subscribe(subscriber)
    }
}

export class Clock2 {
    time: number;
    private _time$: any;
    constructor() {
        this.time = Date.now();
    }

    tick() {
        this.time = Date.now();
        if (this._time$ !== undefined) {
            this._time$.set(this.time)
        }
    }

    get time$() {
        if (this._time$ === undefined) {
            this._time$ = writable(this.time);
        }
        return this._time$
    }
}

