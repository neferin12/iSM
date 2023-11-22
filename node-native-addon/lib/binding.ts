const addon = require('../build/Release/tsism-napi-native');

const runAlgorithm : (choicesPath:string, seminarsPath: string) => string = addon.TsismNapi;

console.log(addon.TsismNapi);
export = addon.TsismNapi

