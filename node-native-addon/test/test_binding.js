const assert = require("assert");
const {loadSeminars, runAlgorithm} = require("../dist/binding");

assert(runAlgorithm, "The expected function is undefined");

function testBasic()
{

    const seminars = loadSeminars("../example-files/Seminare.csv");
    assert(seminars.w_seminars.length > 0, "Empty w seminars");
    assert(seminars.p_seminars.length > 0, "Empty p seminars");
    const result =  runAlgorithm(seminars.w_seminars, seminars.p_seminars, []);
    // assert.strictEqual(result, "world", "Unexpected value returned");
}

assert.doesNotThrow(testBasic, undefined, "testBasic threw an expection");

console.log("Tests passed- everything looks OK!");