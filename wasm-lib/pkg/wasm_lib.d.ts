/* tslint:disable */
/* eslint-disable */
/**
* @param {number} width
* @param {number} height
* @param {number} mines_count
*/
export function start(width: number, height: number, mines_count: number): void;
/**
* @param {number} x
* @param {number} y
*/
export function open(x: number, y: number): void;
/**
* @param {number} x
* @param {number} y
*/
export function flag(x: number, y: number): void;
/**
* @returns {string}
*/
export function fetch(): string;
/**
* @returns {boolean}
*/
export function gameEnded(): boolean;
