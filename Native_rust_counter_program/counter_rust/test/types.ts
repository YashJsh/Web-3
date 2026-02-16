import * as borsh from 'borsh';

export class CounterAccount{
    count : number;
    constructor({count} : {count : number}){
       this.count = count;
    }   
};

export const schema : borsh.Schema = {
    struct : {
        count : 'u32'
    } 
};

export const counter_size = borsh.serialize(
    schema,
    new CounterAccount({count : 0})
).length;

console.log("After serializing : ",borsh.serialize(schema, new CounterAccount({ count : 242})));
console.log("Size to store : ", counter_size);