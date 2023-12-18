const TsismNapi = require("../dist/binding.js");
const assert = require("assert");

assert(TsismNapi, "The expected function is undefined");

function testBasic()
{
    const result =  TsismNapi("hello", "hello2");
    assert.strictEqual(result, "world", "Unexpected value returned");
}

assert.doesNotThrow(testBasic, undefined, "testBasic threw an expection");

console.log("Tests passed- everything looks OK!");