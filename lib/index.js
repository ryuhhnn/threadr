const threadr = require("../native");
const path = require("path");

// This should complete successfully
console.log(threadr.spawn(path.resolve("timeout")));

// This will throw an error
try {
  threadr.spawn(path.resolve("timeoutsss"));
} catch (e) {
  throw e;
}
