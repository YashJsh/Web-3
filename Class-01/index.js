import crypto from "crypto";

const input = "100xdevs";

const hash = crypto.createHash("sha256").update(input).digest("hex");

console.log(hash);

//Code for finding hash starting with 00000
function bruteforce(prefix) {
  let input = 0;
  while (true) {
    let inputStr = input.toString();
    let hash = crypto.createHash("sha256").update(inputStr).digest("hex");
    if (hash.startsWith(prefix)) {
      return {
        input: inputStr,
        hash: hash,
      };
    }
    input++;
  }
}

// TODO : helper function :

// Code for input string to start with the particular thing
// like 100xdevs + hello
function str(string) {
  while (true) {
    let inputStr = "100xdevs" + string;
    let hash = crypto.createHash("sha256").update(inputStr).digest("hex");
    return hash;
  }
}

const result = bruteforce("000000000000");
console.log(result);
const result2 = str("2274885");
console.log(result2);
