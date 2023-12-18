import {Seminar, Student} from "./types";

/**
 * Import the native addon from the build directory.
 */
const addon: {
    runAlgorithm: (w_seminars: Seminar[], p_seminars: Seminar[], students: Student[]) => string,
    loadSeminars: (seminarsPath: string) => null | {w_seminars: Seminar[], p_seminars: Seminar[]},
} = require('../build/Release/tsism-napi-native');

/**
 * Export the runAlgorithm function from the native addon.
 * This function takes two paths as arguments and returns a string.
 * @param {string} choicesPath - The path to the choices file.
 * @param {string} seminarsPath - The path to the seminars file.
 * @returns {string} The result of the algorithm.
 */
export const runAlgorithm = addon.runAlgorithm;

/**
 * Export the loadSeminars function from the native addon.
 * This function takes a path as argument and returns an array of seminars.
 * @param {string} seminarsPath - The path to the seminars file.
 * @returns {{w_seminars: Seminar[], p_seminars: Seminar[]}} The seminars.
 */
export const loadSeminars = addon.loadSeminars;
